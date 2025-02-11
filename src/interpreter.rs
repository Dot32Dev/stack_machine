use crate::token::Instruction;

pub struct Interpreter {
    instructions: Vec<Instruction>,
    stack: Vec<i32>,
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> i32 {
        for instruction in &self.instructions {
            match instruction {
                Instruction::Push(number) => {
                    self.stack.push(*number);
                }
                Instruction::Pop => {
                    self.stack.pop();
                }
                Instruction::Add => {
                    let num2 = self.stack.pop().unwrap();
                    let num1 = self.stack.pop().unwrap();
                    self.stack.push(num1 + num2);
                }
                Instruction::Subtract => {
                    let num2 = self.stack.pop().unwrap();
                    let num1 = self.stack.pop().unwrap();
                    self.stack.push(num1 - num2);
                }
                Instruction::Multiply => {
                    let num2 = self.stack.pop().unwrap();
                    let num1 = self.stack.pop().unwrap();
                    self.stack.push(num1 * num2);
                }
                Instruction::Divide => {
                    let num2 = self.stack.pop().unwrap();
                    let num1 = self.stack.pop().unwrap();
                    self.stack.push(num1 / num2);
                }
            }
        }

        self.stack.pop().unwrap()
    }
}
