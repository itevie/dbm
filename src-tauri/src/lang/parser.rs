use std::mem::{discriminant, Discriminant};

use crate::errors::{MakerError, MakerErrorType};

use super::{
    lexer::{Token, TokenType},
    nodes::{self, Block, Expression},
};

pub struct Parser {
    tokens: Vec<Token>,
}

type E = Result<Expression, MakerError>;

impl Parser {
    fn at(&mut self) -> Token {
        self.tokens.get(0).unwrap().clone()
    }

    fn eat(&mut self) -> Token {
        println!(" eated {:?}", self.at());
        self.tokens.remove(0)
    }

    fn expect<S: Into<String>>(
        &mut self,
        what: Discriminant<TokenType>,
        message: S,
    ) -> Result<Token, MakerError> {
        if discriminant(&mut self.at().token_type) == what {
            return Ok(self.eat());
        } else {
            return Err(MakerError::lang(
                message.into(),
                self.at().location,
                MakerErrorType::ParserError,
            ));
        }
    }

    fn get_identifier(&mut self) -> Result<nodes::Identifier, MakerError> {
        let token = self.expect(
            discriminant(&TokenType::Identifier),
            "Expected an identifier",
        )?;
        match token.token_type {
            TokenType::Identifier => Ok(nodes::Identifier {
                name: token.value,
                location: token.location,
            }),
            _ => unreachable!(),
        }
    }

    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Result<Block, MakerError> {
        let mut block = nodes::Block {
            nodes: vec![],
            location: self.at().location,
        };

        // Repeat until EOF
        while !self.tokens.is_empty() && !matches!(self.at().token_type, TokenType::EOF) {
            block.nodes.push(self.parse_statement()?);
        }

        Ok(block)
    }

    fn parse_statement(&mut self) -> E {
        match self.at().token_type {
            TokenType::Var => self.parse_variable_declaration(),

            TokenType::If => {
                let token = self.eat();
                let test = self.parse_expression()?;
                let block = self.parse_block()?;
                let alternate = if matches!(self.at().token_type, TokenType::Else) {
                    self.eat();
                    if matches!(self.at().token_type, TokenType::Do) {
                        Some(Box::from(Expression::Block(self.parse_block()?)))
                    } else if matches!(self.at().token_type, TokenType::If) {
                        Some(Box::from(self.parse_statement()?))
                    } else {
                        return Err(MakerError::lang(
                            "Expected if or do",
                            self.at().location,
                            MakerErrorType::ParserError,
                        ));
                    }
                } else {
                    None
                };

                Ok(Expression::IfBlock(nodes::IfBlock {
                    test: Box::from(test),
                    success: block,
                    alternate,
                    location: token.location,
                }))
            }
            _ => self.parse_expression(),
        }
    }

    fn parse_variable_declaration(&mut self) -> E {
        let token = self.eat();
        let identifier = self.get_identifier()?;
        self.expect(discriminant(&TokenType::Assign), "Expected =")?;
        let value = self.parse_expression()?;

        Ok(Expression::VariableDeclaration(
            nodes::VariableDeclaration {
                name: identifier,
                value: Box::from(value),
                location: token.location,
            },
        ))
    }

    fn parse_expression(&mut self) -> E {
        match self.at().token_type {
            _ => self.parse_logical_expression(),
        }
    }

    fn parse_logical_expression(&mut self) -> E {
        let left = self.parse_call_expression()?;

        if let TokenType::Logical(logical) = self.at().token_type {
            let operator = self.eat();
            let right = self.parse_call_expression()?;

            return Ok(Expression::Logical(nodes::Logical {
                left: Box::from(left),
                right: Box::from(right),
                location: operator.location,
                operator: logical,
            }));
        }

        Ok(left)
    }

    fn parse_call_expression(&mut self) -> E {
        let left = self.parse_member_expression()?;

        if matches!(self.at().token_type, TokenType::OpenBrace) {
            let start = self.eat();
            let mut args: Vec<Expression> = vec![];

            // Get arguments
            while !self.tokens.is_empty() && !matches!(self.at().token_type, TokenType::CloseBrace)
            {
                args.push(self.parse_expression()?);
                if matches!(self.at().token_type, TokenType::Comma) {
                    self.eat();
                } else {
                    break;
                }
            }

            // Check for ending (
            self.expect(
                discriminant(&TokenType::CloseBrace),
                "Expected closing of arguments",
            )?;

            // Done
            return Ok(Expression::Call(nodes::Call {
                callee: Box::from(left),
                location: start.location,
                args,
            }));
        }

        Ok(left)
    }

    fn parse_member_expression(&mut self) -> E {
        let left = self.parse_literal()?;

        if matches!(self.at().token_type, TokenType::Dot) {
            let start = self.eat();
            let key = self.parse_literal()?;

            return Ok(Expression::Member(nodes::Member {
                left: Box::from(left),
                right: Box::from(key),
                location: start.location,
            }));
        }

        Ok(left)
    }

    fn parse_block(&mut self) -> Result<Block, MakerError> {
        let mut block = nodes::Block {
            nodes: vec![],
            location: self.at().location,
        };

        // Expect a {
        self.expect(discriminant(&TokenType::Do), "Expected do")?;

        // Repeat until {
        while !self.tokens.is_empty() && !matches!(self.at().token_type, TokenType::End) {
            block.nodes.push(self.parse_expression()?);
            println!("{:?}", self.at());
        }

        // Expect a )
        self.expect(discriminant(&TokenType::End), "Expected end")?;

        Ok(block)
    }

    fn parse_literal(&mut self) -> E {
        Ok(match self.at().token_type {
            TokenType::Number => {
                let value = self.eat();

                Expression::Number(nodes::Number {
                    value: value.value.parse::<f64>().unwrap(),
                    location: value.location,
                })
            }
            TokenType::Identifier => {
                let value = self.eat();

                Expression::Identifier(nodes::Identifier {
                    name: value.value,
                    location: value.location,
                })
            }
            TokenType::String => {
                let value = self.eat();

                Expression::StringNode(nodes::StringNode {
                    value: value.value,
                    location: value.location,
                })
            }
            _ => {
                return Err(MakerError::lang(
                    format!("Cannot handle this: {:?}", self.at()),
                    self.at().location,
                    MakerErrorType::ParserError,
                ))
            }
        })
    }
}
