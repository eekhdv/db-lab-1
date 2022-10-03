#[derive(Clone)]
pub enum Menus {
    Main {
        is_test_generated: bool,
        text: Vec<&'static str>,
    },
    Edit {
        tables_list: Vec<String>,
    },
    Create,
    Backup {
        tables_list: Vec<String>,
        backup_path: String,
    },
    Delete {
        tables_list: Vec<String>,
    },
    Unknown,
}

pub enum Keys {
    MainMenuKey,
    CreateTablKey,
    DeleteTablKey,
    BackupTablKey,
    EditTablKey,
    TablListKey,
    GenTestTablKey,
    PrintKey,
    ExitKey,
    UnknownKey,
}

impl Menus {
    pub fn new() -> Menus {
        Menus::Unknown
    }
}
