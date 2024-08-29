use crate::{
    database::{
        database::Database,
        structures::{Bot, CodePiece, Command, Settings},
    },
    discord::runner::{BotRunner, RunnerContext},
};

macro_rules! unwrap {
    ($expr:expr) => {
        match $expr {
            Ok(ok) => Ok(ok),
            Err(err) => {
                println!("{:?}", err);
                Err(err.to_string())
            }
        }
    };
}

// ----- Bot Commands -----

#[tauri::command]
pub async fn get_bot_by_id(id: u8, db: tauri::State<'_, Database>) -> Result<Bot, String> {
    unwrap!(db.inner().bots.get(id).await)
}
#[tauri::command]
pub async fn get_all_bots(db: tauri::State<'_, Database>) -> Result<Vec<Bot>, String> {
    unwrap!(db.inner().bots.get_all_bots().await)
}

#[tauri::command]
pub async fn create_bot(
    name: &str,
    token: &str,
    db: tauri::State<'_, Database>,
) -> Result<Bot, String> {
    unwrap!(db.inner().bots.add(name, token).await)
}

#[tauri::command]
pub async fn set_bot_prefix(
    id: u8,
    prefix: &str,
    db: tauri::State<'_, Database>,
) -> Result<Bot, String> {
    unwrap!(db.inner().bots.set_prefix(id, prefix).await)
}

#[tauri::command]
pub async fn set_bot_name(
    id: u8,
    name: &str,
    db: tauri::State<'_, Database>,
) -> Result<Bot, String> {
    unwrap!(db.inner().bots.set_name(id, name).await)
}

#[tauri::command]
pub async fn set_bot_description(
    id: u8,
    description: &str,
    db: tauri::State<'_, Database>,
) -> Result<Bot, String> {
    unwrap!(db.inner().bots.set_description(id, description).await)
}

#[tauri::command]
pub async fn set_bot_token(
    id: u8,
    token: &str,
    db: tauri::State<'_, Database>,
) -> Result<Bot, String> {
    unwrap!(db.inner().bots.set_token(id, token).await)
}

#[tauri::command]
pub async fn run_bot(
    id: u8,
    db: tauri::State<'_, Database>,
    bot_runner: tauri::State<'_, BotRunner>,
) -> Result<(), String> {
    let bot = unwrap!(db.inner().bots.get(id).await)?;

    bot_runner
        .run(RunnerContext {
            bot,
            database: db.inner().clone(),
        })
        .await;

    Ok(())
}

#[tauri::command]
pub async fn stop_bot(id: u8, bot_runner: tauri::State<'_, BotRunner>) -> Result<(), String> {
    println!("Stoppign bot {}", id);
    bot_runner.stop_bot(id).await;

    Ok(())
}

// ----- Command Commands -----

#[tauri::command]
pub async fn create_command(
    name: &str,
    bot_id: u8,
    db: tauri::State<'_, Database>,
) -> Result<Command, String> {
    unwrap!(db.inner().commands.create(name, bot_id).await)
}

#[tauri::command]
pub async fn get_all_commands(
    bot_id: u8,
    db: tauri::State<'_, Database>,
) -> Result<Vec<Command>, String> {
    unwrap!(db.inner().commands.get_all(bot_id).await)
}

// ----- Code Pieces Commands -----

#[tauri::command]
pub async fn create_code_piece(
    command_id: u8,
    db: tauri::State<'_, Database>,
) -> Result<CodePiece, String> {
    let result = unwrap!(db.inner().code_pieces.create().await)?;
    unwrap!(
        db.inner()
            .commands
            .set_code_piece(command_id, result.id)
            .await
    )?;
    Ok(result)
}

#[tauri::command]
pub async fn get_code_piece(id: u8, db: tauri::State<'_, Database>) -> Result<CodePiece, String> {
    unwrap!(db.inner().code_pieces.get(id).await)
}

#[tauri::command]
pub async fn set_code_piece(
    id: u8,
    code: &str,
    db: tauri::State<'_, Database>,
) -> Result<CodePiece, String> {
    unwrap!(db.inner().code_pieces.set(id, code).await)
}

#[tauri::command]
pub async fn set_command_name(
    id: u8,
    name: &str,
    db: tauri::State<'_, Database>,
) -> Result<Command, String> {
    unwrap!(db.inner().commands.set_name(id, name).await)
}

#[tauri::command]
pub async fn set_command_description(
    id: u8,
    description: &str,
    db: tauri::State<'_, Database>,
) -> Result<Command, String> {
    unwrap!(db.inner().commands.set_description(id, description).await)
}

// ----- Settings Commands -----

#[tauri::command]
pub async fn get_options(db: tauri::State<'_, Database>) -> Result<Settings, String> {
    unwrap!(db.inner().options.get_options().await)
}

#[tauri::command]
pub async fn set_selected_bot(
    bot_id: u8,
    db: tauri::State<'_, Database>,
) -> Result<Settings, String> {
    unwrap!(db.inner().options.set_selected_bot(bot_id).await)
}
