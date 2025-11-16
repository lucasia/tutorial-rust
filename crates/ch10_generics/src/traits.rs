use log::debug;

pub fn traits() {
    // TODO - completed up to Listing 10-13: Implementing the Summary trait on the NewsArticle and SocialPost types

    debug!("====== traits ======");

    let headline = "According to our research...";
    let author = "George Washington";
    let content = "You AUTO buy a Honda";

    let article = NewsArticle::new(headline, author, content);

    assert_eq!(article.headline, headline);
    assert_eq!(article.author, author);
    assert_eq!(article.content, content);

    debug!("{}", article.summarize());

    let social_post = SocialPost {
        username: author.to_string(),
        content: content.to_string(),
    };

    debug!("{}", social_post.summarize());
    notify(&social_post);
}

pub fn notify(item: &impl Summary) {
    // syntactic sugar of notify<T: Summary>(item: &T)
    debug!("Breaking news! {}", item.summarize());
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl NewsArticle {
    pub fn new(headilne: &str, author: &str, content: &str) -> Self {
        NewsArticle {
            headline: headilne.to_string(),
            author: author.to_string(),
            content: content.to_string(),
        }
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.to_string()
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
