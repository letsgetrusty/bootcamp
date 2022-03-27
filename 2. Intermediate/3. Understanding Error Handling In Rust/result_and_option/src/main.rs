use std::{fs, io};

fn main() {
    let first_line = read_first_line("example.txt");
}

fn read_first_line(filename: &str) -> Result<Option<String>, io::Error> {
    fs::read_to_string(filename).map(|s| {
        s.lines().next().map(|s| s.to_owned())
    })
}

fn read_first_line2(filename: &str) -> Option<String> {
    fs::read_to_string(filename).ok().and_then(|s| {
        s.lines().next().map(|s| s.to_owned())
    })
}