#[tokio::main]
async fn main() {
    let f = my_function();
    println!("Let's Get Rusty!");
    f.await;
}

async fn my_function() {
    println!("I'm an async function!");
    let s1 = read_from_database().await;
    println!("First result: {s1}");
    let s2 = read_from_database().await;
    println!("Second result: {s2}");
}

async fn read_from_database() -> String {
    "DB Result".to_owned()
}