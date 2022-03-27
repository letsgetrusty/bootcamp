unsafe trait MyTrait {
    fn some_function(&self);
}

unsafe impl MyTrait for String {
    fn some_function(&self) {
        // ...
    }
}

fn main() {
   let s = "some string".to_owned();
   s.some_function();
}
