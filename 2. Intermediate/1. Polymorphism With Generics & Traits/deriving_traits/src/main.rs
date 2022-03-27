#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 3, y: 1 };
    let p2 = Point { x: 3, y: 1 };
    let p3 = Point { x: 5, y: 5 };

    println!("{:?}", p1);
    println!("{}", p1 == p2);
    println!("{}", p1 == p3);
}