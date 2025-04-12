use crate::token::Token;
use std::iter::Peekable;
use std::mem;

pub struct Lexer<'a> {
    chars: Peekable<std::str::Chars<'a>>,
    line_number: u32,
    token: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a String) -> Self {
        let mut lexer = Lexer {
            chars: content.chars().peekable(),
            line_number: 1,
            token: None,
        };
        lexer.token = lexer.get_token();
        lexer
    }

    pub fn next(&mut self) -> Option<Token> {
        let token = self.get_token();
        // Moves `token` into `self.token` and returns what used to be in there
        mem::replace(&mut self.token, token)
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.token.as_ref()
    }

    fn get_token(&mut self) -> Option<Token> {
        while let Some(ch) = self.chars.next() {
            match ch {
                '\n' => {
                    self.line_number += 1;
                    continue;
                }
                ' ' | '\t' => continue,
                '0'..='9' => {
                    let mut integer = ch.to_string();
                    while let Some(&peek) = self.chars.peek() {
                        if !peek.is_digit(10) {
                            break;
                        }
                        integer.push(self.chars.next().unwrap());
                    }
                    let value = integer.parse::<i32>().unwrap();
                    return Some(Token::Integer(value));
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut variable = ch.to_string();
                    while let Some(&peek) = self.chars.peek() {
                        if !peek.is_alphabetic() {
                            break;
                        }
                        variable.push(self.chars.next().unwrap());
                    }
                    return Some(Token::Variable(variable));
                }
                '+' => return Some(Token::Add),
                '-' => return Some(Token::Subtract),
                '*' => return Some(Token::Multiply),
                '/' => return Some(Token::Divide),
                '(' => return Some(Token::LeftParen),
                ')' => return Some(Token::RightParen),
                ';' => return Some(Token::Semicolon),
                '=' => return Some(Token::Equals),
                _ => println!("Unrecognised character"),
            }
        }

        None
    }
}
