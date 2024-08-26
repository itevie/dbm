use crate::errors::{MakerError, MakerErrorType};

#[derive(Debug, Clone)]
pub enum TokenType {
    // ----- Literals -----
    Identifier,
    Number,
    String,

    // ----- Symbols -----
    Dot,
    Comma,
    Assign,
    OpenBrace,
    CloseBrace,
    OpenCurly,
    CloseCurly,

    // ----- Keywords -----
    Var,
    Do,
    End,
    If,

    // ----- Special -----
    EOF,
    None,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub location: Location,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Location {
    pub start: u16,
    pub end: u16,
    pub line: u16,
    pub context: String,
}

impl Location {
    pub fn no_location() -> Location {
        Location {
            start: 0,
            end: 0,
            line: 0,
            context: "<unknown>".to_string(),
        }
    }
}

macro_rules! eat {
    ($a:ident, $b:ident) => {{
        $a += 1;
        $b.remove(0)
    }};
}

pub fn lex(contents: String, context: String) -> Result<Vec<Token>, MakerError> {
    let mut chars: Vec<char> = contents.chars().collect();
    let mut tokens: Vec<Token> = vec![];

    let mut current_char: u16 = 0;
    let mut current_line: u16 = 0;

    while !chars.is_empty() {
        let mut location = Location {
            start: current_char,
            end: current_char,
            line: current_line,
            context: context.clone(),
        };

        let mut token_value: Option<String> = None;
        let mut token_type: Option<TokenType> = None;

        let mut set_token = |value, t| {
            token_value = Some(value);
            token_type = Some(t);
        };

        match chars[0] {
            // Whitespace
            ' ' | '\t' | '\r' => {
                eat!(current_char, chars);
                continue;
            }
            // Newlines
            '\n' => {
                chars.remove(0);
                current_char = 0;
                current_line += 1;
                continue;
            }
            // Identifiers
            _ if chars[0].is_alphabetic() || chars[0] == '_' => {
                let mut value = eat!(current_char, chars).to_string();

                // Repeat until not alphanumeric
                while !chars.is_empty() && (chars[0].is_alphanumeric() || chars[0] == '_') {
                    value.push(eat!(current_char, chars));
                }

                // Check if it is a keyword
                let kw: Option<TokenType> = match value.as_str() {
                    "var" => Some(TokenType::Var),
                    "do" => Some(TokenType::Do),
                    "end" => Some(TokenType::End),
                    "if" => Some(TokenType::If),
                    _ => None,
                };

                // Add it
                if let Some(kw) = kw {
                    set_token(value, kw);
                } else {
                    set_token(value, TokenType::Identifier);
                }
            }
            // Numbers
            _ if chars[0].is_numeric() => {
                let mut value = eat!(current_char, chars).to_string();

                // Repeat until not numeric
                while !chars.is_empty() && chars[0].is_numeric() {
                    value.push(eat!(current_char, chars));
                }

                set_token(value, TokenType::Number);
            }
            // Strings
            '"' => {
                eat!(current_char, chars);
                let mut value = String::new();

                // Repeat until a "
                while !chars.is_empty() && chars[0] != '"' {
                    value.push(eat!(current_char, chars));
                }

                // Make sure there was a "
                if chars.is_empty() || chars[0] != '"' {
                    return Err(MakerError::lang(
                        "Expected end of string",
                        location,
                        MakerErrorType::RuntimeError,
                    ));
                }

                eat!(current_char, chars);

                // Set it
                set_token(value, TokenType::String)
            }
            // Others
            _ => {
                let symbol_type = match chars[0] {
                    '(' => TokenType::OpenBrace,
                    ')' => TokenType::CloseBrace,
                    '{' => TokenType::OpenCurly,
                    '}' => TokenType::CloseCurly,
                    '.' => TokenType::Dot,
                    ',' => TokenType::Comma,
                    '=' => TokenType::Assign,
                    _ => TokenType::None,
                };

                // Check if nothing was found
                if matches!(symbol_type, TokenType::None) {
                    return Err(MakerError::lang(
                        format!("Unexpected character: {}", chars[0]),
                        location,
                        MakerErrorType::LexerError,
                    ));
                }

                // Add it
                set_token(eat!(current_char, chars).to_string(), symbol_type);
            }
        }

        // Add token
        if matches!(token_value, None) || matches!(token_type, None) {
            return Err(MakerError::lang(
                "An unknown error occurred".to_string(),
                location,
                MakerErrorType::LexerError,
            ));
        }

        location.end = current_char;

        tokens.push(Token {
            value: token_value.unwrap(),
            token_type: token_type.unwrap(),
            location,
        });
    }

    // Add EOF
    tokens.push(Token {
        value: "".to_string(),
        token_type: TokenType::EOF,
        location: Location {
            start: current_char,
            end: current_char,
            line: current_line,
            context,
        },
    });

    Ok(tokens)
}
