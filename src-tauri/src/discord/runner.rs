use std::collections::HashMap;
use std::sync::Arc;

use lang::{lexer::lex, parser::Parser, runtime::Interpreter, scope::Scope};
use log::info;
use serenity::all::GatewayIntents;
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task; // Ensure this is imported

use crate::database::database::Database;
use crate::database::structures::Bot;
use crate::database::structures::CodePiece;
use crate::database::structures::Command;
use crate::discord::native_functions::generate_from_message;
use crate::errors::MakerError;
use crate::errors::MakerErrorType;
use crate::lang;
use crate::rs2js;
use crate::rs2js::tx_error;
use crate::rs2js::SenderType;

struct Handler {
    commands: Vec<Command>,
    code_pieces: Vec<CodePiece>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        let prefix = "!";

        // Guards
        if msg.author.bot {
            return;
        }

        // Get the content & args
        let content: &str = &(&msg.content.to_lowercase())[prefix.len()..];
        let mut args = content.split(' ').collect::<Vec<&str>>();
        let cmd_name = args.remove(0);

        // Try to find command
        let cmd = self.commands.iter().find(|x| x.name == cmd_name);

        if let Some(cmd) = cmd {
            info!("Executing command {}", cmd.name);

            // Get the code contents
            let contents = self
                .code_pieces
                .iter()
                .find(|x| x.id == cmd.code_id)
                .unwrap();

            // Lex & Parse
            let lexed = lex(contents.code.clone(), "test".to_string());
            let parsed = Parser::new(lexed.unwrap()).parse().unwrap();
            let mut scope = Scope::new();

            // Get the funcs for the message
            let funcs = generate_from_message(Arc::from(_ctx), Arc::from(msg));
            scope.declare("message", funcs).unwrap();

            // Run it
            let mut interpreter = Interpreter::new(scope);
            interpreter
                .evaluate(lang::nodes::Expression::Block(parsed))
                .await
                .unwrap();
        }
    }
}

#[derive(Clone)]
pub struct RunnerContext {
    pub database: Database,
    pub bot: Bot,
}

pub struct BotRunner {
    running_bots: Arc<tokio::sync::Mutex<HashMap<u8, (task::JoinHandle<()>, oneshot::Sender<()>)>>>,
    tx: Arc<mpsc::Sender<SenderType>>,
}

impl BotRunner {
    pub fn new(tx: mpsc::Sender<SenderType>) -> BotRunner {
        BotRunner {
            running_bots: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            tx: Arc::new(tx),
        }
    }

    pub async fn run(&self, context: RunnerContext) {
        // Check if it is already started
        if self.running_bots.lock().await.contains_key(&context.bot.id) {
            tx_error!(
                self.tx,
                format!("Bot {} already running", context.bot.name),
                BotRunnerError
            );
            info!("Bot is already running: {}", context.bot.id);
            return;
        }

        info!("Attempting to start bot {}", context.bot.id);

        let commands = context
            .database
            .commands
            .get_all(context.bot.id)
            .await
            .unwrap();
        let code_pieces = context
            .database
            .code_pieces
            .get_all(context.bot.id)
            .await
            .unwrap();

        let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
        let (stop_tx, stop_rx) = oneshot::channel();

        let context2 = context.clone();
        let running_bots = self.running_bots.clone();
        let tx = self.tx.clone();

        // Spawn a new task for the bot
        let bot_task = tokio::spawn(async move {
            let mut client = Client::builder(context2.bot.token.clone(), intents)
                .event_handler(Handler {
                    commands,
                    code_pieces,
                })
                .await
                .expect("Error creating client");

            let result = tokio::select! {
                r = client.start() => {
                    // Handle client started
                    r
                },
                _ = stop_rx => {
                    // Handle stop signal
                    info!("Received stop signal on bot {}", context2.bot.id);
                    client.shard_manager.shutdown_all().await;
                    Ok(())
                }
            };

            if let Err(err) = result {
                tx_error!(
                    tx,
                    format!("Bot error: {}", err.to_string()),
                    BotRunnerError
                );
            }

            // Remove running bot from the list
            {
                let mut bots = running_bots.lock().await;
                bots.remove(&context2.bot.id);

                tx.send(SenderType::RunningBotsUpdate(rs2js::RunningBotsUpdate {
                    list: bots.keys().cloned().collect(),
                }))
                .await
                .unwrap();
            }
        });

        // Insert the bot into the running_bots map
        {
            let mut bots = self.running_bots.lock().await;
            bots.insert(context.bot.id, (bot_task, stop_tx));

            self.tx
                .send(SenderType::RunningBotsUpdate(rs2js::RunningBotsUpdate {
                    list: bots.keys().cloned().collect(),
                }))
                .await
                .unwrap();
        }
    }

    pub async fn stop_bot(&self, bot_id: u8) {
        info!("Attempting to stop bot {}", bot_id);
        let removed = {
            let mut things = self.running_bots.lock().await;
            things.remove(&bot_id)
        };

        if let Some((_, stop_tx)) = removed {
            // Send a stop signal to the bot task
            let _ = stop_tx.send(()); // Ignore errors if the receiver is already dropped

            self.tx
                .send(SenderType::RunningBotsUpdate(rs2js::RunningBotsUpdate {
                    list: self.running_bots.lock().await.keys().cloned().collect(),
                }))
                .await
                .unwrap();
        } else {
            info!("No bot with ID {} was running.", bot_id);
        }
    }
}
