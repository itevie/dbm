#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct Settings {
    pub current_bot: Option<u8>,
}

#[derive(sqlx::FromRow, Debug, serde::Serialize, Clone)]
pub struct Bot {
    pub id: u8,
    pub name: String,
    pub token: String,
    pub description: String,
}

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct Command {
    pub id: u8,
    pub name: String,
    pub bot_id: u8,
    pub code_id: u8,
}

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct CodePiece {
    pub id: u8,
    pub code: String,
}
