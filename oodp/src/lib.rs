// Blog post workflow
// 1. A blog post starts as an empty draft
// 2. When the draft is done, a review of the post is requested
// 3. When the post is approved, it gets published
// 4. Only published blog posts return content to print,
//    so unapproved posts can’t accidentally be published

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // 1. A blog post starts as an empty draft
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // 2. Do the draft
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // 2. When the draft is done, a review of the post is requested
    pub fn request_review(&mut self) {
        // we call the take method to take the Some value out of the state field
        // and leave a None in its place
        // This lets us move the state value out of Post rather than borrowing it
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    // 3. When the post is approved, it gets published
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
        // If we didn’t call as_ref, we would get an error
        // because we can’t move state out of the borrowed
        // &self of the function parameter
        // Then call the unwrap method, which we know will never panic,
        // because we know the methods on Post ensure that
        // state will always contain a Some value when those methods are done
        // When we know that a None value is never possible,
        // even though the compiler isn’t able to understand that.
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// add a new struct that implements State, the Draft state
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}


// add a new struct that implements State, the PendingReview state
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// add a new struct that implements State, the Published state
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// The implementation using the state pattern is easy to extend to add more functionality
// To see the simplicity of maintaining code that uses the state pattern,
// try a few of these suggestions:

// 1. Add a reject method that changes the post’s state from PendingReview back to Draft
// 2. Require two calls to approve before the state can be changed to Published
// 3. Allow users to add text content only when a post is in the Draft state
//    Hint: have the state object responsible for what might change about the content
//          but not responsible for modifying the Post.