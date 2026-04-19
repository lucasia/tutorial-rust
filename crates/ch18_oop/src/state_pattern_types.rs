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
            content: String,
        }

        pub struct DraftPost {
            content: String,
        }

        pub struct PendingReviewPost {
            content: String,
        }

        impl Post {
            pub fn new() -> DraftPost {
                DraftPost {
                    content: String::new(),
                }
            }

            pub fn content(&self) -> &str {
                &self.content
            }
        }

        impl DraftPost {
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }

            pub fn request_review(self) -> PendingReviewPost {
                PendingReviewPost {
                    content: self.content,
                }
            }
        }

        impl PendingReviewPost {
            pub fn approve(self) -> Post {
                Post {
                    content: self.content,
                }
            }
        }
    }

    #[test]
    fn test_blog_post() {
        let mut post = blog::Post::new();

        let text = "I ate a salad for lunch today";
        post.add_text(text);

        let post = post.request_review();

        let post = post.approve();
        assert_eq!(text, post.content());
    }
}
