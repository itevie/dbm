use std::{error::Error, fmt};

use serde::ser::SerializeStruct;

use crate::lang::lexer::Location;

#[derive(serde::Serialize, Debug, Clone)]
pub enum MakerErrorType {
    UnknownDatabase,
    BotAlreadyExists,
    LexerError,
    ParserError,
    RuntimeError,
    BotRunnerError,
}

#[derive(Debug, Clone)]
pub struct MakerError {
    pub message: String,
    pub error_type: MakerErrorType,
    pub source: Option<String>,
    pub location: Option<Location>,
}

unsafe impl Send for MakerError {}

impl fmt::Display for MakerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{:?}] Maker Error: {}: {:?}",
            self.error_type, self.message, self.source
        )
    }
}

impl MakerError {
    pub fn from(error: Option<Box<dyn Error>>, error_type: MakerErrorType) -> Self {
        Self {
            message: if let Some(ref error) = error {
                error.to_string()
            } else {
                "An error occured".to_string()
            },
            source: error.map(|x| x.to_string()),
            error_type,
            location: None,
        }
    }

    pub fn new<S: Into<String>>(message: S, error_type: MakerErrorType) -> Self {
        Self {
            message: message.into(),
            source: None,
            error_type,
            location: None,
        }
    }

    pub fn lang<S: Into<String>>(
        message: S,
        location: Location,
        error_type: MakerErrorType,
    ) -> Self {
        Self {
            location: Some(location),
            message: message.into(),
            source: None,
            error_type,
        }
    }
}

impl Error for MakerError {}

// Stolen from ChatGPT
impl serde::Serialize for MakerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("MakerError", 2)?;

        state.serialize_field("message", &self.message)?;
        state.serialize_field("error_type", &self.error_type)?;

        // Check for source
        if let Some(source) = &self.source {
            state.serialize_field("source", &source.to_string())?;
        } else {
            state.serialize_field("source", &None::<String>)?;
        }

        // Check for location
        if let Some(location) = &self.location {
            state.serialize_field("location", &location)?;
        } else {
            state.serialize_field("location", &None::<String>)?;
        }

        state.end()
    }
}
