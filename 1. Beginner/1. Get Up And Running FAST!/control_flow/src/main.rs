fn main() {
    // if/else
    let a = 5;

    if a > 5 {
        println!("bigger than 5");
    } else if a > 3 {
        println!("bigger than 3");
    } else {
        println!("smaller or equal to 3");
    }

    let b = if a > 5 { 1 } else { -1 };

    // loop
    let x = loop {
        break 5;
    };

    // while loop
    let mut a = 0;

    while a < 5 {
        println!("a is {a}");
        a = a + 1;
    }

    // for loop
    let a = [1, 2, 3, 4, 5];
    
    for element in a {
        println!("{}", element);
    }
}
