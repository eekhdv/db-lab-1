// mod.rs
use core::arch::global_asm;
use std::fs;
use std::mem::drop;

global_asm!(include_str!("random.asm"));

extern "sysv64" {
    fn system_rand(seed: i64, module: i64) -> i64;
}

// extern "efiapi" {
//     fn efi_rand(seed: i64) -> i64;
// }
// extern "win64" {
//     fn win64_rand(seed: i64) -> i64;
// }

pub mod randi64 {
    fn asm_random(module: i64) -> u32 {
        let seed = rand::random::<i64>();
        let prime: u32 = 289035269;
        let res;

        unsafe {
            res = system_rand(seed, module);
        }
        res as u32 % prime
    }
}

pub mod tablmgr {
    use std::fs::write;
    use std::fs::OpenOptions;
    use std::path::Path;

    fn add(_tbl: String, data: String) {
        let output = Path::new(format!("../../static_data/{}.txt", _tbl).as_str());
        let file = OpenOptions::new()
            .append(true)
            .open(output)
            .expect("[ERROR] unable to open file");
        let new_data = data.trim().replace(" ", ",");

        write(output, new_data).expect("[ERROR] unable to wirte file");
        drop(file);
    }

    fn edit(_tbl: String, id: u32) {
        let output = Path::new(format!("../../static_data/{}.txt", _tbl).as_str());
        let file = OpenOptions::new()
            .write(true)
            .open(output)
            .expect("[ERROR] unable to open file");
    }

    fn del(_tbl: String, id: u64) -> String {
        let output = Path::new(format!("../../static_data/{}.txt", _tbl).as_str());
        let file = OpenOptions::new()
            .write(true)
            .open(output)
            .expect("[ERROR] unable to open file");
    }

    fn print(_tbl: String, id: u32) -> String {
        let output = Path::new(format!("../../static_data/{}.txt", _tbl).as_str());
        let file = OpenOptions::new()
            .read(true)
            .open(output)
            .expect("[ERROR] unable to open file");
    }
}
