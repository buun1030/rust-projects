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
    assert_eq!("", post.content());

    // 2. When the draft is done, a review of the post is requested
    post.request_review();
    assert_eq!("", post.content());

    // 3. When the post is approved, it gets published
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}