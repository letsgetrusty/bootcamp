fn main() {
    let s1 = String::from("Rust"); // heap allocated string
    let s2 = s1.clone();

    println!("s1 is: {s1}");

    let x = 10;
    let y = x;
    println!("x is: {x}");
}
