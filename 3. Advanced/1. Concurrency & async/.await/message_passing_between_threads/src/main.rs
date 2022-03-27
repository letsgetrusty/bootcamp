use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let sentences = [
        "!dlroW olleH".to_owned(),
        ".tsurT eW tsuR nI".to_owned(),
        "!ytsuR teG s'teL".to_owned(),
        "!tsuB ro tsuR".to_owned(),
    ];

    for s in sentences {
        let tx_clone = tx.clone();
        
        thread::spawn(move || {
            let reversed: String = s.chars().rev().collect();
            tx_clone.send(reversed).unwrap();
        });
    }

    drop(tx);

    for sentence in rx {
        println!("{sentence}");
    }
}