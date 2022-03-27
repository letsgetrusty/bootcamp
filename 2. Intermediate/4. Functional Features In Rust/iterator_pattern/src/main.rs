trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

trait IntoIterator {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}

struct MyStruct {}

impl Iterator for MyStruct {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn main() {
    let mut m = MyStruct {};
    let item = m.next();
}