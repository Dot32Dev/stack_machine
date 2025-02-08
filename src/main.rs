use eframe::egui;
use parser::Parser;
use std::fs;

mod lexer;
mod parser;
mod token;

struct MyApp {
    name: String,
    age: u32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't read file");
    let _ = Parser::new(&input).expr();
    print!("\n")

    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default()
    //         .with_inner_size([320.0, 240.0]),
    //     ..Default::default()
    // };

    // eframe::run_native(
    //     "My egui App",
    //     options,
    //     Box::new(|cc| {
    //         // This gives us image support:
    //         // egui_extras::install_image_loaders(&cc.egui_ctx);

    //         Ok(Box::new(MyApp {
    //             name: String::new(),
    //             age: 0,
    //         }))
    //     }),
    // )
    // .unwrap();
}
