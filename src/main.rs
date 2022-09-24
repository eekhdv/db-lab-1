use core::arch::global_asm;

global_asm!(include_str!("random.asm"));

// extern "efiapi" {
//     fn efi_rand(seed: i64) -> i64;
// }
// extern "win64" {
//     fn win64_rand(seed: i64) -> i64;
// }

extern "sysv64" {
    fn system_rand(seed: i64) -> i64;
}

fn asm_random() -> u32 {
    let seed = rand::random::<i64>();
    let prime: u32 = 289035269;
    let res;

    unsafe {
        res = system_rand(seed);
    }
    res as u32 % prime
}

fn main() {
    println!("{}", asm_random());
}
