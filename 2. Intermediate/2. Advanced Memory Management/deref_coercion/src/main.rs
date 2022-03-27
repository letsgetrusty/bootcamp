use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> MySmartPointer<T> {
        MySmartPointer { value }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let mut s = MySmartPointer::new(
        Box::new("Let's Get Rusty".to_owned())
    );
    // let s = &(***s);

    // &MySmartPointer -> &Box -> &String -> &str
    print(&mut s);
}

fn print(s: &str) {
    println!("{s}");
}
