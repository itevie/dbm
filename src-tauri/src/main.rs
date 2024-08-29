#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod discord;
mod errors;
mod lang;
mod rs2js;
mod tauri_commands;

use discord::runner::BotRunner;
use log::{Level, LevelFilter, Metadata, Record};
use rs2js::SenderType;
use tauri::{generate_handler, Manager};
use tauri_commands::*;
use tokio::sync::mpsc;

use database::database::Database;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if !record.metadata().target().starts_with("dbm::") {
                return;
            }

            println!(
                "[{}: {}] - {}",
                record.level(),
                record.metadata().target(),
                record.args(),
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

#[tokio::main]
async fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::max()))
        .unwrap();

    let (async_proc_input_tx, _async_proc_input_rx) = mpsc::channel::<SenderType>(1);
    let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel::<SenderType>(1);

    let database = Database::new().await;
    let bot_runner = BotRunner::new(async_proc_output_tx.clone());

    let _ = tauri::Builder::default()
        .manage(async_proc_input_tx)
        .manage(database)
        .manage(bot_runner)
        .invoke_handler(generate_handler![
            get_bot_by_id,
            create_bot,
            get_options,
            get_all_bots,
            create_command,
            set_selected_bot,
            get_all_commands,
            create_code_piece,
            set_code_piece,
            get_code_piece,
            run_bot,
            stop_bot,
            set_bot_description,
            set_bot_name,
            set_bot_prefix,
            set_command_name,
            set_command_description,
            set_bot_token
        ])
        .setup(|app| {
            let app_handle = app.handle();
            let window = app_handle.get_window("main").unwrap();

            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(ref output) = async_proc_output_rx.recv().await {
                        println!("{:?}", output);
                        window.emit(output.get_sender_id(), output).unwrap();
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while generating tauri application");
}
