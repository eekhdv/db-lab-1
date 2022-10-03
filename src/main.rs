mod logic;
use std::{
    collections::VecDeque,
    fs::File,
    io::{Read, Write},
    num::ParseIntError,
    path::Path,
};

use eframe::{
    egui::{Context, Grid, TopBottomPanel, Ui, Window},
    epaint::Vec2,
};

fn main() {
    let mut line = String::new();
    println!("Enter qwerty");
    std::io::stdin().read_line(&mut line).unwrap();
    if line.replace("\n", "").eq("qwerty") {
        // logic::tablgen::gen_test_table();
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
        let mut buf = String::new();
        let mut full_names = String::new();

        let path = Path::new("../generated_tables/testing_table.txt");
        let static_names = Path::new("../static_data/names.txt");

        File::open(path).unwrap().read_to_string(&mut buf).unwrap();
        File::open(static_names)
            .unwrap()
            .read_to_string(&mut full_names)
            .unwrap();

        TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.heading(get_table_name(
                path.file_name().unwrap().to_str().unwrap().to_string(),
            ));
        });

        Window::new("Test table window")
            .vscroll(true)
            .show(ctx, |ui| {
                let mut temp_file = logic::tablmgr::create(".temp".to_string(), 'a');

                let buf_names = full_names.split("\n");
                let buf_tests = buf.split("\n");

                let mut data1 = buf_names.clone().into_iter();
                let mut data2 = buf_tests.clone().into_iter();

                let mut rows: VecDeque<Vec<String>> = VecDeque::new();

                let length = buf_tests.count();
                for _ in 0..length - 1 {
                    let data2write = format!(
                        "{},{}\n",
                        data1.next().unwrap().replace("\r", ""),
                        data2.next().unwrap()
                    );
                    temp_file.write(data2write.as_bytes()).unwrap();
                }

                let mut temp_data = String::new();
                File::open(std::path::Path::new("../generated_tables/.temp.txt"))
                    .unwrap()
                    .read_to_string(&mut temp_data)
                    .unwrap();

                for i in temp_data.split("\n") {
                    let mut inner_vec: Vec<String> = Vec::new();
                    for j in i.split(',') {
                        inner_vec.push(j.to_string());
                    }
                    rows.push_back(inner_vec);
                }

                // for i in buf_tests {
                //     let mut var: String = String::new();
                //     match i.split(',').nth(1) {
                //         Some(x) => var = x.to_string(),
                //         None => println!("[ERROR]"),
                //     }
                //     test_vars.push_back(var);
                // }
                table_test_grid(ui, rows);
            });
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

fn table_test_grid(ui: &mut Ui, mut rows: VecDeque<Vec<String>>) {
    Grid::spacing(Grid::new("general_table"), Vec2::new(10.0, 1.0)).show(ui, |ui| {
        ui.heading("id");

        let mut counter = 1;
        for item in rows.pop_front().unwrap() {
            ui.heading(item);
        }
        ui.end_row();

        for row in rows {
            ui.label(counter.to_string());
            for item in row {
                ui.label(item);
            }
            ui.end_row();
            counter += 1;
        }
    });
}
