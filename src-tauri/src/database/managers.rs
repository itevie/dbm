use log::info;
use sqlx::sqlite::SqlitePool;

use crate::errors::{MakerError, MakerErrorType};

use super::structures::{Bot, CodePiece, Command, Settings};

// Thanks ChatGPT :3c
macro_rules! execute_query {
    // Pattern with no bindings
    ($self:expr, $t:ty, $query:expr, $fetch_fn:ident) => {{
        info!("Executing SQL query: {}", $query);
        sqlx::query_as::<_, $t>($query)
            .$fetch_fn(&$self.pool)
            .await
            .map_err(|err| MakerError::from(Some(Box::from(err)), MakerErrorType::UnknownDatabase))
    }};

    // Pattern with one or more bindings
    ($self:expr, $t:ty, $query:expr, $fetch_fn:ident, $( $bind:expr ),*) => {{
        info!("Executing SQL query: {}", $query);
        {
            let mut query = sqlx::query_as::<_, $t>($query);
            $(
                query = query.bind($bind);
            )*
            query
        }
        .$fetch_fn(&$self.pool)
        .await
        .map_err(|err| MakerError::from(Some(Box::from(err)), MakerErrorType::UnknownDatabase))
    }};
}

#[derive(Clone)]
pub struct BotManager {
    pub pool: SqlitePool,
}

impl BotManager {
    pub async fn get(&self, id: u8) -> Result<Bot, MakerError> {
        execute_query!(
            self,
            Bot,
            "SELECT * FROM bots WHERE id = ?1;",
            fetch_one,
            id
        )
    }

    pub async fn get_all_bots(&self) -> Result<Vec<Bot>, MakerError> {
        execute_query!(self, Bot, "SELECT * FROM bots;", fetch_all)
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Bot, MakerError> {
        execute_query!(
            self,
            Bot,
            "SELECT * FROM bots WHERE name = ?1;",
            fetch_one,
            name
        )
    }

    pub async fn add(&self, name: &str, token: &str) -> Result<Bot, MakerError> {
        // Check if the bot already exists
        if let Ok(_) = self.get_by_name(name).await {
            return Err(MakerError {
                message: format!("Bot {} already exists", name),
                error_type: MakerErrorType::BotAlreadyExists,
                source: None,
                location: None,
            });
        }

        execute_query!(
            self,
            Bot,
            "INSERT INTO bots (name, token) VALUES (?1, ?2) RETURNING *;",
            fetch_one,
            name,
            token
        )
    }

    pub async fn set_prefix(&self, bot_id: u8, prefix: &str) -> Result<Bot, MakerError> {
        execute_query!(
            self,
            Bot,
            "UPDATE bots SET prefix = ?2 WHERE id = ?1 RETURNING *;",
            fetch_one,
            bot_id,
            prefix
        )
    }

    pub async fn set_name(&self, bot_id: u8, name: &str) -> Result<Bot, MakerError> {
        execute_query!(
            self,
            Bot,
            "UPDATE bots SET name = ?2 WHERE id = ?1 RETURNING *;",
            fetch_one,
            bot_id,
            name
        )
    }

    pub async fn set_description(&self, bot_id: u8, description: &str) -> Result<Bot, MakerError> {
        execute_query!(
            self,
            Bot,
            "UPDATE bots SET description = ?2 WHERE id = ?1 RETURNING *;",
            fetch_one,
            bot_id,
            description
        )
    }

    pub async fn set_token(&self, bot_id: u8, token: &str) -> Result<Bot, MakerError> {
        execute_query!(
            self,
            Bot,
            "UPDATE bots SET token = ?2 WHERE id = ?1 RETURNING *;",
            fetch_one,
            bot_id,
            token
        )
    }
}

#[derive(Clone)]
pub struct SettingsManager {
    pub pool: SqlitePool,
}

impl SettingsManager {
    pub async fn get_options(&self) -> Result<Settings, MakerError> {
        execute_query!(self, Settings, "SELECT * FROM settings;", fetch_one)
    }

    pub async fn set_selected_bot(&self, bot_id: u8) -> Result<Settings, MakerError> {
        execute_query!(
            self,
            Settings,
            "UPDATE settings SET current_bot = ?1 RETURNING *;",
            fetch_one,
            bot_id
        )
    }
}

#[derive(Clone)]
pub struct CommandManager {
    pub pool: SqlitePool,
}

impl CommandManager {
    pub async fn create(&self, name: &str, bot_id: u8) -> Result<Command, MakerError> {
        execute_query!(
            self,
            Command,
            "INSERT INTO commands (name, bot_id) VALUES (?1, ?2) RETURNING *;",
            fetch_one,
            name,
            bot_id
        )
    }

    pub async fn get_all(&self, bot_id: u8) -> Result<Vec<Command>, MakerError> {
        execute_query!(
            self,
            Command,
            "SELECT * FROM commands WHERE bot_id = ?1;",
            fetch_all,
            bot_id
        )
    }

    pub async fn set_code_piece(&self, id: u8, code_id: u8) -> Result<Command, MakerError> {
        execute_query!(
            self,
            Command,
            "UPDATE commands SET code_id = ?1 WHERE id = ?2 RETURNING *;",
            fetch_one,
            code_id,
            id
        )
    }

    pub async fn set_name(&self, command_id: u8, name: &str) -> Result<Command, MakerError> {
        execute_query!(
            self,
            Command,
            "UPDATE commands SET name = ?2 WHERE id = ?1 RETURNING *;",
            fetch_one,
            command_id,
            name
        )
    }

    pub async fn set_description(
        &self,
        command_id: u8,
        description: &str,
    ) -> Result<Command, MakerError> {
        execute_query!(
            self,
            Command,
            "UPDATE commands SET description = ?2 WHERE id = ?1 RETURNING *;",
            fetch_one,
            command_id,
            description
        )
    }
}

#[derive(Clone)]
pub struct CodePieceManager {
    pub pool: SqlitePool,
}

impl CodePieceManager {
    pub async fn create(&self) -> Result<CodePiece, MakerError> {
        execute_query!(
            self,
            CodePiece,
            "INSERT INTO code_pieces DEFAULT VALUES RETURNING *;",
            fetch_one
        )
    }

    pub async fn _create_with(&self, code: &str) -> Result<CodePiece, MakerError> {
        execute_query!(
            self,
            CodePiece,
            "INSERT INTO code_pieces (code) VALUES (?1) RETURNING *;",
            fetch_one,
            code
        )
    }

    pub async fn get(&self, id: u8) -> Result<CodePiece, MakerError> {
        execute_query!(
            self,
            CodePiece,
            "SELECT * FROM code_pieces WHERE id = ?1;",
            fetch_one,
            id
        )
    }

    pub async fn get_all(&self, bot_id: u8) -> Result<Vec<CodePiece>, MakerError> {
        execute_query!(
            self,
            CodePiece,
            r#"SELECT cp.*
                FROM code_pieces cp
                JOIN commands c ON cp.id = c.code_id
                WHERE c.bot_id = ?1;"#,
            fetch_all,
            bot_id
        )
    }

    pub async fn set(&self, id: u8, code: &str) -> Result<CodePiece, MakerError> {
        execute_query!(
            self,
            CodePiece,
            "UPDATE code_pieces SET code = ?2 WHERE id = ?1 RETURNING *;",
            fetch_one,
            id,
            code
        )
    }
}
