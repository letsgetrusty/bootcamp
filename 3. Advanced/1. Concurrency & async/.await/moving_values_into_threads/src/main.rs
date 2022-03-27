use std::thread;

fn main() {
    let s = "Let's Get Rusty!".to_owned();

    let handle = thread::spawn(move || {
        println!("{s}");
    });
}