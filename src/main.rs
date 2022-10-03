use colored::Colorize;
use std::io::Write;

mod gui;
mod logic;

#[derive(Clone)]
enum Menus {
    Main {
        is_test_generated: bool,
        text: Vec<&'static str>,
    },
    Edit {
        tables_list: Vec<String>,
    },
    Create,
    Delete,
}

fn main() {
    let mut generated = false;
    let mut log = String::new();
    let mut buf;
    loop {
        let mut tables: Vec<String> = Vec::new();
        let mut current_menu = Menus::Main {
            is_test_generated: generated,
            text: vec![
                "1. Create table.",
                "2. Delete table.",
                "3. Edit table.",
                "4. Table list.",
                "-------------------------",
                "8. Generate testing table.",
                "9. Print testing table.",
                "-------------------------",
                "10. Exit (or q).",
            ],
        };

        let paths = std::fs::read_dir("../generated_tables/").unwrap();
        for path in paths {
            tables.push(gui::tablgui::get_table_name(
                path.unwrap().file_name().into_string().unwrap(),
            ));
        }
        buf = menu_to_show(current_menu.clone(), log.clone());
        println!("{}", buf);
        match buf.replace("\n", "").to_lowercase().as_str() {
            "1" => (),
            "2" => (),
            "3" => {}
            "4" => {
                log = format!("{}", get_tables_list(tables));
            }
            "8" => {
                generated = true;
                logic::tablgen::gen_test_table();
                log = format!(
                    "{:30}",
                    "TESTING TABLE GENERATED!".bold().green().on_black()
                );
            }
            "9" => {
                if generated {
                    logic::tablmgr::create(".temp".to_string(), 'a');
                    let options = eframe::NativeOptions::default();
                    eframe::run_native(
                        "Informational table",
                        options,
                        Box::new(|_cc| Box::new(gui::tablgui::MyApp)),
                    );
                }
            }
            "10" | "q" | "exit" => {
                match logic::tablmgr::clean() {
                    Ok(()) => (),
                    Err(_) => (),
                };
                break;
            }
            _ => {
                log = format!("{:30}", "TRY ONE MORE TIME!".bold().bright_red().on_black());
            }
        };
    }
}

fn get_tables_list(tables: Vec<String>) -> String {
    let mut log;
    let mut counter = 0;
    if tables.is_empty() {
        log = format!(
            "{:30}",
            "THERE ARE NO TABLES".bold().bright_red().on_black()
        );
    } else {
        log = format!("{:30}", "TABLES LIST:".green().bold().on_black()) + "\n";
        for i in tables {
            log += counter.to_string().as_str();
            log += ". ";
            log += i.as_str();
            log += "\n";
            counter += 1;
        }
    }
    log
}

fn menu_to_show(menus: Menus, log: String) -> String {
    print!("{}[2J", 27 as char);
    let mut res = String::new();
    match menus {
        Menus::Main {
            is_test_generated: generated,
            text: text_print,
        } => {
            // let mut buf = String::new();
            for i in text_print {
                if i.eq("9. Print testing table.") {
                    if generated {
                        println!("{}", i);
                    }
                } else {
                    println!("{}", i);
                }
            }
            // println!("1. Create table.");
            // println!("2. Delete table.");
            // println!("3. Edit table.");
            // println!("4. table list.");
            // println!("-------------------------");
            // println!("8. Generate testing table.");
            // if generated {
            //     println!("9. Print testing table.");
            // }
            // println!("-------------------------");
            // println!("10. Exit (or q).");
            // println!("{}", log);

            print!("> ");
            std::io::stdout().flush().unwrap();

            std::io::stdin().read_line(&mut res).unwrap();
        }
        Menus::Edit {
            tables_list: tables,
        } => {
            println!("Choose which table do you want to edit?");
            println!("{}", get_tables_list(tables));
        }
        Menus::Create => (),
        Menus::Delete => (),
    }
    res
}
