use c17_state_pattern::Post;

fn main() {
    let mut post = Post::new();
    
    post.add_text("I ate a salad for lunch today");
    println!("Added text to the post.");
    assert_eq!("", post.content());

    post.request_review();
    println!("Post review requested.");
    assert_eq!("", post.content());

    post.approve();
    println!("Post approved for review.");
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("Post content: {}", post.content());
}