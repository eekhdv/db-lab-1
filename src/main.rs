use std::{collections::VecDeque, fs::File, io::Read, path::Path};

use eframe::{
    egui::{Context, Grid, TopBottomPanel, Ui, Window},
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

        Window::new("My Window").vscroll(true).show(ctx, |ui| {
            let buf_names = full_names.split("\n");

            let mut rows: VecDeque<Vec<String>> = VecDeque::new();
            for i in buf_names {
                let mut inner_vec: Vec<String> = Vec::new();
                for j in i.split(",") {
                    inner_vec.push(j.to_string());
                }
                rows.push_back(inner_vec);
            }
            table_test_grid(ui, rows);
        });
    }
}
/* CentralPanel::default().show(ctx, |ui| {
    let buf_names = full_names.split("\n");
    // let buf_lines = buf.split("\n").into_iter();

    let text_style = TextStyle::Body;
    let row_height = ui.text_style_height(&text_style);
    let total_rows = buf_names.clone().into_iter().count();

    // let mut buf_full_names = ;

    ScrollArea::vertical().show_rows(ui, row_height, total_rows, |ui, row_range| {
        let mut rows: Vec<Vec<String>> = Vec::new();
        for i in buf_names {
            let mut inner_vec: Vec<String> = Vec::new();
            for j in i.split(",") {
                inner_vec.push(j.to_string());
            }
            rows.push(inner_vec);
        }
        table_test_grid(ui, rows, row_range);
        // for row in row_range {
        //     let text = format!("Row {}/{}", row + 1, total_rows);
        //     ui.label(text);
        //     ui.end_row();
        // }
        // TODO: show_rows
    });
});*/

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

pub fn table_test_grid(ui: &mut Ui, mut rows: VecDeque<Vec<String>>) {
    Grid::spacing(Grid::new("general_table"), Vec2::new(10.0, 1.0)).show(ui, |ui| {
        for item in rows.pop_front().unwrap() {
            ui.heading(item);
        }
        ui.end_row();
        for row in rows {
            for item in row {
                ui.label(item);
            }
            ui.end_row();
        }
    });
}
// let mut buf_full_names = ;
// let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.

/* let mut buf_lines = buf.split("\n").into_iter();
* let mut buf_names = full_names.split("\n").into_iter();
* // let mut buf_full_names = ;
* for i in buf_names.next().unwrap().split(",") {
*     ui.heading(i);
* }
* ui.heading(buf_lines.next().unwrap().split(",").nth(1).unwrap());
* ui.end_row();

* // TODO: printing to gui

* for i in buf_lines {
*     // i => stdnt_id , vrnt_id
*     // let stdnt_id = i
*     //     .split(",")
*     //     .nth(0)
*     //     .unwrap()
*     //     .to_string()
*     //     .parse::<usize>()
*     //     .unwrap();
*     let mut a = buf_names.next();
*     if a != None {
*         ui.label(a.unwrap());
*     }
*     // let full_name = buf_names
*     //     .nth(qq.nth(0).unwrap().to_string().parse::<usize>().unwrap())
*     //     .unwrap();
*     // for k in full_name.split(",").into_iter() {
*     //     ui.label(k);
*     // }
*     // ui.verticaj_centered(|ui| {
*     //     // ui.jabel(buf_full_names.next().unwrap());
*     //     // ui.label(buf_full_names.next().unwrap());
*     //     // ui.label(buf_full_names.next().unwrap());
*     //     ui.label(j);
*     // });
*     // ui.label(j);
*     // ui.label(qq.nth(1).unwrap());
*     ui.end_row();
* }
*/

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
