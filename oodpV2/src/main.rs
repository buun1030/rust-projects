// Blog post workflow
// 1. A blog post starts as an empty draft
// 2. When the draft is done, a review of the post is requested
// 3. When the post is approved, it gets published
// 4. Only published blog posts return content to print,
//    so unapproved posts can’t accidentally be published

use blog::Post;

fn main() {
    // 1. A blog post starts as an empty draft
    let mut post = Post::new();

    // 2. Do the draft
    post.add_text("I ate a salad for lunch today");
    // we can’t compile code that tries to use the content
    // of posts in those states any longer

    // 2. When the draft is done, a review of the post is requested
    let post = post.request_review();

    // 3. When the post is approved, it gets published
    let post = post.approve();
    println!("{}",post.content());
    assert_eq!("I ate a salad for lunch today", post.content());
}