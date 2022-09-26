#[derive(Debug)]

pub struct Student {
    id: u8,
    name: String,
    surname: String,
    patronymic: String,
}

pub struct Variant {
    id: u8,
    path: String,
}

impl Student {
    pub fn new(id: u8, name: String, surname: String, patronymic: String) -> Student {
        Student(id, name, surname, patronymic)
    }
}

impl Variant {
    pub fn new(id: u8, path: String) -> Student {
        Variant(id, path)
    }
}
