use crate::token::Token;
use std::iter::Peekable;

pub struct Lexer<'a> {
    chars: Peekable<std::str::Chars<'a>>,
    line_number: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a String) -> Self {
        let chars = content.chars().peekable();
        Lexer {
            chars,
            line_number: 1,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.chars.next() {
            match ch {
                // Increment line count, then skip
                '\n' => {
                    self.line_number += 1;
                    continue;
                }
                // Skip whitespace
                ' ' | '\t' => continue,
                // Collect an integer number
                '0'..='9' => {
                    let mut integer = String::new();
                    integer.push(ch);
                    'number: loop {
                        match self.chars.peek() {
                            Some(peek) if peek.is_digit(10) => {
                                let ch = self.chars.next().expect("Impossible");
                                integer.push(ch);
                            }
                            Some(_) => break 'number,
                            None => break 'number
                        }
                    }
                    let value = integer.parse::<i32>().unwrap();
                    return Some(Token::Integer(value));
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut variable = String::new();
                    variable.push(ch);
                    'var: loop {
                        match self.chars.peek() {
                            Some(peek) if peek.is_alphanumeric() => {
                                let ch = self.chars.next().expect("Impossible");
                                variable.push(ch);
                            }
                            Some(_) => break 'var,
                            None => break 'var
                        }
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
