use std::arch::asm;

fn add(x: u64, y: u64) -> u64 {
    let result: u64;

    unsafe {
        // x86/x86-64 assembly
        asm!("add {0}, {1}", inout(reg) x => result, in(reg) y);
    }

    result
}

fn main() {}
