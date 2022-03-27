fn main() {
    // Example 1
    let player1 = String::from("player 1");
    let player2 = String::from("player 2");

    let result = first_turn(player1.as_str(), player2.as_str());

    // How does the borrow checker know result is not a dangling reference?
    println!("Player going first is: {}", result);

    // Example 2
    let player1 = String::from("player 1");
    {
        let player2 = String::from("player 2");
        let result = first_turn(player1.as_str(), player2.as_str());
        println!("Player going first is: {}", result);
    }

    // Example 3
    let player1 = String::from("player 1");
    let result;
    {
        let player2 = String::from("player 2");
        result = first_turn(player1.as_str(), player2.as_str());
    }
    println!("Player going first is: {}", result);
}

fn first_turn(p1: &str, p2: &str) -> &str {
    if rand::random() {
        p1
    } else {
        p2
    }
}
