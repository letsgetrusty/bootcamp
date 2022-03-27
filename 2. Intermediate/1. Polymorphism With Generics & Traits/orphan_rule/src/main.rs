use orphan_rule::Point;

struct PointWrapper(Point);

impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

fn main() {
    let p1 = PointWrapper(Point { x: 1, y: 2 });
    let p2 = PointWrapper(Point { x: 1, y: 2 });
    println!("{}", p1 == p2);
}
