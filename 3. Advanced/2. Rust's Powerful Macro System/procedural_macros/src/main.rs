use procedural_macros::*;

trait Log {
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);
}

#[derive(Debug, Log)]
struct Database {
    url: String,
    connections: u32,
}

impl Database {
    fn new(url: String) -> Database {
        Database { url, connections: 0 }
    }
    fn connect(&mut self) {
        self.info(format!("new connection to {}", self.url).as_str());
        self.connections += 1;
        if self.connections >= 100 {
            self.warn(format!("100 or more connections open!").as_str());
        }
    }
}

#[derive(Debug)]
struct Product {
    name: String,
    price: u32
}

fn main() {
    let laptop = Product { name: "MacBook Pro".to_owned(), price: 2000 };
    buy_product(laptop, 20);
}

#[log_call(verbose)]
fn buy_product(product: Product, discount: u32) {
    // ...
}