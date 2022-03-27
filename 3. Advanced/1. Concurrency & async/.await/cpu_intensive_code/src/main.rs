use std::time::Duration;
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    handles.push(tokio::spawn(async {
        let _res = tokio::task::spawn_blocking(|| {
            expensive_computation();
        }).await;
    }));

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_function(i: i32) {
    println!("[{i}] I'm an async function!");
    let s1 = read_from_database().await;
    println!("[{i}] First result: {s1}");
    let s2 = read_from_database().await;
    println!("[{i}] Second result: {s2}");
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_owned()
}

fn expensive_computation() {
    let mut i = 0;
    for _ in 0..400_000_000 {
        i = i + 1;
    }
    println!("Done with expensive computation! i = {i}");
}