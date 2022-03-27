#[link(name = "adder", kind="static")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let x: i32;
    unsafe {
        x = add(1, 2);
    }
    println!("x is: {x}");
}
