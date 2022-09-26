// mod.rs
use core::arch::global_asm;

global_asm!(include_str!("random.asm"));

extern "sysv64" {
    pub fn system_rand(seed: i64, module: i64) -> i64;
}

// extern "efiapi" {
//     fn efi_rand(seed: i64) -> i64;
// }
// extern "win64" {
//     fn win64_rand(seed: i64) -> i64;
// }

pub mod randi64 {
    pub fn asm_random(module: i64) -> u32 {
        let seed = rand::random::<i64>();
        let prime: u32 = 289035269;
        let res: i64;

        unsafe {
            res = system_rand(seed, module);
        }
        res as u32 % prime
    }
}

pub mod tablmgr {
    use std::fs::{write, File, OpenOptions};
    use std::io::{Read, Write};
    use std::path::Path;

    fn add(_tbl: String, data: String) -> Result<(), std::io::Error> {
        let output = Path::new(format!("../../static_data/{}.txt", _tbl).as_str());

        let file = OpenOptions::new()
            .append(true)
            .open(output)
            .expect("[ERROR] unable to open file");
        let new_data = data.trim().replace(" ", ",");

        write(output, new_data).expect("[ERROR] unable to wirte file");
        drop(file);
        Ok(())
    }

    fn edit(_tbl: String, id: u32, new_data: String) -> Result<(), std::io::Error> {
        let output = Path::new(format!("../../static_data/{}.txt", _tbl).as_str());

        let mut src = File::open(output).expect("[ERROR] unable to open file");
        let mut old_lines = String::new();
        src.read_to_string(&mut old_lines);
        drop(src);

        let new_line = old_lines.replace(
            old_lines
                .split('\n')
                .collect::<Vec<_>>()
                .get(id as usize)
                .expect("[ERROR] element with this id doesn't exist"),
            &new_data.replace(" ", ","),
        );
        let mut dst = File::open(output).expect("[ERROR] unable to open file");
        dst.write(new_line.as_bytes())
            .expect("[ERROR] unable write to the file");
        Ok(())
    }

    fn del(_tbl: String, id: u64) -> String {
        let output = Path::new(format!("../../static_data/{}.txt", _tbl).as_str());

        let mut src = File::open(output).expect("[ERROR] unable to open file");
        let mut old_lines = String::new();
        src.read_to_string(&mut old_lines);
        drop(src);

        let deleted_line = format!(
            "{}\n",
            old_lines
                .split('\n')
                .collect::<Vec<_>>()
                .get(id as usize)
                .expect("[ERROR] element with this id doesn't exist")
        );
        let data = old_lines.replace(deleted_line.as_str(), "");

        let mut dst = File::open(output).expect("[ERROR] unable to open file");
        dst.write(data.as_bytes())
            .expect("[ERROR] unable write to the file");

        deleted_line
    }

    fn print(_tbl: String, id: u32) -> String {
        let output = Path::new(format!("../../static_data/{}.txt", _tbl).as_str());

        let mut src = File::open(output).expect("[ERROR] unable to open file");
        let mut lines = String::new();
        src.read_to_string(&mut lines);

        lines
            .split('\n')
            .collect::<Vec<_>>()
            .get(id as usize)
            .expect("[ERROR] element with this id doesn't exist")
            .replace(",", " ")
            .to_string()
    }
}
