fn main() {    
    // Example 1
    let s1 = String::from("Let's Get Rusty!");
    println!("s1: {s1}");
    let s2 = s1;
    println!("s1: {s1}");

    // Example 2
    let r1;
    {
        let s1 = String::from("Let's Get Rusty!");
        r1 = &s1;
    }
    
    println!("r1: {r1}");

    //Example 3 (non-lexical)
    let mut s1 = String::from("Let's Get Rusty");
    let r1 = &s1;  
    println!("r1: {r1}");
    let r2 = &mut s1;
    r2.push_str("!");
}
