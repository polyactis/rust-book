use c17_state_pattern::Post;

fn main() {
    let mut post = Post::new();
    
    post.add_text("I ate a salad for lunch today");
    println!("Added text to the post.");
    println!("Post content: {}", post.content());
    assert_eq!("", post.content());

    post.request_review();
    println!("Post review requested.");
    println!("Post content: {}", post.content());
    assert_eq!("", post.content());

    post.reject();
    println!("Post review rejected.");
    println!("Post content: {}", post.content());

    post.request_review();
    println!("Post review requested.");
    println!("Post content: {}", post.content());
    assert_eq!("", post.content());

    post.approve();
    println!("Post approved for review.");
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("Post content: {}", post.content());
}