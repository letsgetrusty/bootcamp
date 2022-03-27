use blog_shared::Post;

fn main() {
    let post = Post::new(
        "Post on the web".to_owned(),
        "Let's Get Rusty!".to_owned(),
    );
    
    println!("{post:?}");
}
