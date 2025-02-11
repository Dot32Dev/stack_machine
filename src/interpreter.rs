use crate::token::Instruction;

pub struct Interpreter {
    pub instructions: Vec<Instruction>,
    pub stack: Vec<i32>,
    pub instruction_pointer: usize,
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            stack: Vec::new(),
            instruction_pointer: 0,
        }
    }

    pub fn next(&mut self) {
        match self.instructions[self.instruction_pointer] {
            Instruction::Push(number) => {
                self.stack.push(number);
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

        self.instruction_pointer += 1;
    }

    pub fn run(&mut self) -> i32 {
        for _ in 0..self.instructions.len() {
            self.next();
        }

        self.stack.pop().unwrap()
    }
}
