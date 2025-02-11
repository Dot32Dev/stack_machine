use eframe::egui;
use egui::Color32;
use egui::RichText;
use interpreter::Interpreter;
use parser::Parser;
use std::fs;
use std::time::{Duration, Instant};
use token::Instruction;

mod interpreter;
mod lexer;
mod parser;
mod token;

struct MyApp {
    interpreter: Interpreter,
    last_update: Instant,
    source: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        let now = Instant::now();
        if now.duration_since(self.last_update) >= Duration::from_millis(1000) {
            if self.interpreter.instructions.len()
                > self.interpreter.instruction_pointer
            {
                self.interpreter.next();
                self.last_update = now;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Expression");
            ui.label(&self.source);
            ui.add_space(10.0);
            ui.heading("Stack Machine");
            ui.columns(2, |cols| {
                // for (i, col) in cols.iter_mut().enumerate() {
                //     // col.label(format!("Column {} out of {}", i + 1, 2));
                //     // // if i + 1 == 2 && col.button("Delete this").clicked() {
                //     // //     //
                //     // // }
                //     if
                // }
                cols[0].label("Instructions");
                cols[0].add_space(10.0);
                for (i, instruction) in
                    self.interpreter.instructions.iter().enumerate()
                {
                    let string = match instruction {
                        Instruction::Push(number) => {
                            format!("Push( {} )", number)
                        }
                        Instruction::Pop => String::from("Pop"),
                        Instruction::Add => String::from("Add"),
                        Instruction::Subtract => String::from("Subtract"),
                        Instruction::Multiply => String::from("Multiply"),
                        Instruction::Divide => String::from("Divide"),
                    };
                    if i == self.interpreter.instruction_pointer {
                        cols[0].label(
                            RichText::new(string)
                                .color(Color32::from_rgb(110, 255, 110)),
                        );
                    } else {
                        cols[0].label(string);
                    }
                }

                cols[1].label("Stack");
                cols[1].add_space(10.0);
                for number in &self.interpreter.stack {
                    cols[1].label(number.to_string());
                }
            });
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Increment").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't read file");
    let mut parser = Parser::new(&input);
    parser.expr();
    let mut interpreter = Interpreter::new(parser.instructions);
    // let answer = interpreter.run();
    // println!("{}", answer);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 280.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Stack machine",
        options,
        Box::new(|_cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(MyApp {
                interpreter,
                last_update: Instant::now(),
                source: input,
            }))
        }),
    )
    .unwrap();
}
