#[cfg(test)]
mod tests {
    use test_log::test;

    // states
    // draft -> review -> published
    // A blog post starts as an empty draft.
    // When the draft is done, a review of the post is requested.
    // When the post is approved, it gets published.
    // Only published blog posts return content to print so that unapproved posts can’t accidentally be published.

    pub mod blog {
        pub struct Post {
            state: Option<Box<dyn State>>,
            content: String,
        }

        trait State {
            // self: Box<Self> means this method takes ownership of the Box, consuming the current
            // state. This allows us to return a brand new state, effectively replacing ourselves.
            fn request_review(self: Box<Self>) -> Box<dyn State>;
        }

        struct Draft {}
        struct PendingReview {}

        impl State for Draft {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                Box::new(PendingReview {})
            }
        }

        impl State for PendingReview {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }

        impl Post {
            pub fn approve(&self) {
                todo!()
            }
        }

        impl Post {
            pub fn content(&self) -> &str {
                // TODO - need state logic, for now just return empty str
                ""
            }

            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }

            pub fn new() -> Post {
                Post {
                    state: Some(Box::new(Draft {})),
                    content: String::new(),
                }
            }

            pub fn request_review(&mut self) {
                // self.state is Option<Box<dyn State>>. We can't call request_review() while the state
                // is still inside self (borrow conflict), so take() swaps it to None giving us ownership.
                // We consume the old state, get back a new one, and restore it.
                if let Some(s) = self.state.take() {
                    self.state = Some(s.request_review())
                }
            }
        }
    }

    #[test]
    fn test_blog_post() {
        let mut post = blog::Post::new();

        let text = "I ate a salad for lunch today";
        post.add_text(text);
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        // TODO - add post approve logic!, see: https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html#adding-approve-to-change-contents-behavior
        post.approve();
        assert_eq!(text, post.content());
    }
}
