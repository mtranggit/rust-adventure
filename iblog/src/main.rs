use iblog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I had Vietnamese bread roll Banh Mi Thit for lunch today");
    // assert_eq!("", post.content());

    let post = post.request_review();
    // assert_eq!("", post.content());

    let post = post.approve();
    assert_eq!(
        "I had Vietnamese bread roll Banh Mi Thit for lunch today",
        post.content()
    );
}
