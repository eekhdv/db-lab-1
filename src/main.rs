use std::io::Read;

use eframe::{
    egui::{CentralPanel, Context, Grid, TopBottomPanel},
    epaint::Vec2,
};

fn main() {
    let mut line = String::new();
    println!("Enter qwerty");
    std::io::stdin().read_line(&mut line).unwrap();
    if line.replace("\n", "").eq("qwerty") {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "Informational table",
            options,
            Box::new(|_cc| Box::new(MyApp)),
        );
    }
}

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let path = std::path::Path::new("../generated_tables/testing_table.txt");
        let mut buf = String::new();
        std::fs::File::open(path)
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();

        TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.heading(get_table_name(
                path.file_name().unwrap().to_str().unwrap().to_string(),
            ));
        });
        CentralPanel::default().show(ctx, |ui| {
            // ui.heading(get_table_name(
            //     path.file_name().unwrap().to_str().unwrap().to_string(),
            // ));
            let test = Grid::spacing(Grid::new("general_table"), Vec2::new(20.0, 1.0));
            test.show(ui, |ui| {
                let mut buf_lines = buf.split("\n").into_iter();
                for i in buf_lines.next().unwrap().split(",") {
                    ui.heading(i);
                }
                ui.end_row();

                for i in buf_lines {
                    for j in i.split(",").into_iter() {
                        ui.vertical_centered(|ui| {
                            ui.label(j);
                        });
                        // ui.label(j);
                    }
                    ui.end_row();
                }

                // ui.label("First row, first column");
                // ui.label("First row, second column");
                // ui.end_row();

                // ui.label("Second row, first column");
                // ui.label("Second row, second column");
                // ui.label("Second row, third column");
                // ui.end_row();

                // ui.horizontal(|ui| {
                //     ui.label("Same");
                //     ui.label("cell");
                // });
                // ui.label("Third row, second column");
                // ui.end_row();
            });
        });
        // egui::CentralPanel::default().show(ctx, |ui| {
        //     ui.heading("My egui Application");
        //     ui.horizontal(|ui| {
        //         ui.label("Your name: ");
        //         ui.text_edit_singleline(&mut self.name);
        //     });
        //     ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
        //     if ui.button("Click each year").clicked() {
        //         self.age += 1;
        //     }
        //     ui.label(format!("Hello '{}', age {}", self.name, self.age));
        // });
    }
}

fn get_table_name(file_name: String) -> String {
    file_name
        .replace(".txt", "")
        .split("_")
        .into_iter()
        .map(|x| {
            let mut tmp = x.chars();
            match tmp.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + tmp.as_str() + " ",
            }
        })
        .collect::<String>()
}
