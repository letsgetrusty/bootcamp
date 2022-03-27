use std::{fs, num::ParseIntError, error};

fn main() {
    let i = parse_file("example.txt");
    match i {
        Ok(i) => println!("{i}"),
        Err(e) => {
            match e {
                ParseFileError::File => {
                    // ...
                },
                ParseFileError::Parse(e) => {
                    // ...
                }
            }
        }
    }
}

// fn parse_file(filename: &str) -> Result<i32, Box<dyn error::Error>> {
//     let s = fs::read_to_string(filename)?;
//     let i = s.parse()?;
//     Ok(i)
// }

enum ParseFileError {
    File,
    Parse(ParseIntError),
}

fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename)
                    .map_err(|e| ParseFileError::File)?;
    let i = s.parse()
                    .map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}