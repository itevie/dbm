use std::{fs, path::PathBuf};

use super::managers::{BotManager, CodePieceManager, CommandManager, SettingsManager};
use log::info;
use sqlx::{sqlite::SqlitePool, Executor};

#[derive(Clone)]
pub struct Database {
    pub bots: BotManager,
    pub options: SettingsManager,
    pub commands: CommandManager,
    pub code_pieces: CodePieceManager,
}

impl Database {
    pub async fn new() -> Self {
        let mut path = PathBuf::new();
        path.push("../data.db");

        if !path.exists() {
            fs::write(path.clone(), "").unwrap();
        }

        let path_str = path.canonicalize().unwrap().display().to_string();
        info!("The database path is: {}", path_str);

        // Create the pool
        let pool = SqlitePool::connect(&format!("sqlite:{}", path_str))
            .await
            .unwrap();

        // Execute initial query
        if let Err(err) = pool.execute(include_str!("../main.sql")).await {
            panic!("Failed to execute init script: {}", err);
        }

        // Done
        Database {
            bots: BotManager { pool: pool.clone() },
            options: SettingsManager { pool: pool.clone() },
            commands: CommandManager { pool: pool.clone() },
            code_pieces: CodePieceManager { pool: pool.clone() },
        }
    }
}
