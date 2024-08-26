use crate::errors::MakerError;

macro_rules! tx_error {
    ($tx:expr,$msg:expr,$type:ident) => {
        $tx.send(SenderType::Error(rs2js::Error {
            error: MakerError::new($msg, MakerErrorType::$type),
        }))
        .await
        .unwrap();
    };
}

pub(crate) use tx_error;

#[derive(serde::Serialize, Clone, Debug)]
pub enum SenderType {
    RunningBotsUpdate(RunningBotsUpdate),
    Error(Error),
}

impl SenderType {
    pub fn get_sender_id(&self) -> &str {
        match self {
            SenderType::RunningBotsUpdate(_) => "running_bots_update",
            SenderType::Error(_) => "error",
        }
    }
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct RunningBotsUpdate {
    pub list: Vec<u8>,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct Error {
    pub error: MakerError,
}
