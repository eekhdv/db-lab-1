use core::arch::global_asm;

global_asm!(include_str!("random.asm"));


extern "sysv64" {
    fn random() -> i64;
}


fn main() {
    println!("{}", unsafe{random()});
}
