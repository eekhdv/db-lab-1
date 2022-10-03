mod gui;
mod logic;

fn main() {
    logic::tablmgr::create(".temp".to_string(), 'a');
    logic::tablgen::gen_test_table();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Informational table",
        options,
        Box::new(|_cc| Box::new(gui::tablgui::MyApp)),
    );
    match logic::tablmgr::clean() {
        Ok(()) => eprintln!("[INFO] Generated tables clean successfully!"),
        Err(_) => eprintln!("[ERROR] Can't clean generated tables"),
    };
    return;
}
