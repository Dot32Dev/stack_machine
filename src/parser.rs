use eframe::egui::ahash::HashMapExt;

use crate::{lexer::Lexer, token::Instruction, token::Token};
// use std::collections::HashMap;

pub struct Parser<'a> {
    lexer: std::iter::Peekable<Lexer<'a>>,
    // symtable: HashMap<String, f32>, // Changed to owned String
    pub instructions: Vec<Instruction>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a String) -> Self {
        let lexer = Lexer::new(content).peekable();

        Self {
            lexer,
            // symtable: HashMap::new(),
            instructions: Vec::new(),
        }
    }

    pub fn expr(&mut self) {
        // An expression is a term plus/minus another term
        self.term();
        let peek = self.lexer.peek();
        match peek {
            Some(Token::Add) | Some(Token::Subtract) => {
                let op = self.lexer.next();
                self.term();

                match op {
                    Some(Token::Add) => {
                        self.instructions.push(Instruction::Add)
                    }
                    Some(Token::Subtract) => {
                        self.instructions.push(Instruction::Subtract)
                    }
                    _ => unreachable!(),
                }
            }
            _ => (),
        }
    }

    fn term(&mut self) {
        // A term is a thingmo multiplied/divided by another thingmo
        // a thingmo can be a brackets thing

        self.factor();
        let peek = self.lexer.peek();
        match peek {
            Some(Token::Multiply) | Some(Token::Divide) => {
                let op = self.lexer.next();
                self.factor();

                match op {
                    Some(Token::Multiply) => {
                        self.instructions.push(Instruction::Multiply)
                    }
                    Some(Token::Divide) => {
                        self.instructions.push(Instruction::Divide)
                    }
                    _ => unreachable!(),
                }
            }
            _ => (),
        }
    }

    fn factor(&mut self) {
        // we lookin for a number or an opening bracket
        let peek = self.lexer.peek();
        match peek {
            Some(Token::Integer(number)) => {
                self.instructions.push(Instruction::Push(*number));
                self.lexer.next();
            }
            Some(Token::LeftParen) => {
                self.lexer.next();
                self.expr();
                self.lexer.next(); // assume there's a closing bracket
            }
            _ => (),
        }
    }
}
