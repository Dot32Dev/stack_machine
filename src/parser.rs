use crate::{lexer::Lexer, token::Instruction, token::Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    pub instructions: Vec<Instruction>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a String) -> Self {
        let lexer = Lexer::new(content);

        Self {
            lexer,
            instructions: Vec::new(),
        }
    }

    pub fn expr(&mut self) {
        // An expression is a term with moreterms
        self.term();
        self.more_terms();
    }

    pub fn term(&mut self) {
        // A term is a factor with morefactors
        self.factor();
        self.more_factors();
    }

    pub fn more_terms(&mut self) {
        // More terms is a plus/minus with a term and then another moreterms
        // Can also be empty
        let peek = self.lexer.peek();
        match peek {
            Some(Token::Add) => {
                self.lexer.next();
                self.term();
                self.instructions.push(Instruction::Add);
                self.more_terms();
            }
            Some(Token::Subtract) => {
                self.lexer.next();
                self.term();
                self.instructions.push(Instruction::Subtract);
                self.more_terms();
            }
            _ => (),
        }
    }

    pub fn factor(&mut self) {
        // A factor is a number or brackets with an expression inside
        let peek = self.lexer.peek();
        match peek {
            Some(Token::Integer(number)) => {
                self.instructions.push(Instruction::Push(*number));
                self.lexer.next();
            }
            Some(Token::LeftParen) => {
                self.lexer.next();
                self.expr();
                // Assume there's a closing bracket
                // TODO: Error checking
                self.lexer.next();
            }
            _ => (),
        }
    }

    pub fn more_factors(&mut self) {
        // More factors is a mul/div with a factor and then another morefactors
        // Can also be empty
        let peek = self.lexer.peek();
        match peek {
            Some(Token::Multiply) => {
                self.lexer.next();
                self.factor();
                self.instructions.push(Instruction::Multiply);
                self.more_factors();
            }
            Some(Token::Divide) => {
                self.lexer.next();
                self.factor();
                self.instructions.push(Instruction::Divide);
                self.more_factors();
            }
            _ => (),
        }
    }
}
