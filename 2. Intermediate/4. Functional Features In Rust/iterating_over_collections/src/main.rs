use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("red team".to_owned(), 2);
    scores.insert("blue team".to_owned(), 8);
    scores.insert("green team".to_owned(), 6);

    // let mut score_iter = scores.iter();
    // let first = score_iter.next();

    // let mut score_iter = scores.iter_mut();
    // let first = score_iter.next();

    // let mut score_iter = scores.into_iter();
    // let first = score_iter.next();

    // for (team, score) in &scores {
    //     println!("{team} Got: {score} points");
    // }

    // for (team, score) in &mut scores {
    //     println!("{team} Got: {score} points");
    // }

    for (team, score) in scores {
        println!("{team} Got: {score} points");
    }
}