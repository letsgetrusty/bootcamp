use std::collections::HashMap;
use declarative_macros::*;

fn main() {
    hello!();

    let scores: HashMap<String, i32> = HashMap::new();
    let scores = map!(String, i32);

    let mut scores2 = HashMap::new();
    scores2.insert("Red team".to_owned(), 3);
    scores2.insert("Blue team".to_owned(), 5);
    scores2.insert("Green team".to_owned(), 2);

    let scores2 = map!(
        "Red team".to_owned() => 3,
        "Blue team".to_owned() => 5,
        "Green team".to_owned() => 2
    );
}