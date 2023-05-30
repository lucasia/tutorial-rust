use std::fmt::Display;

pub trait Summary {
    // fn summarize(&self) -> String; // no impl

    fn summarize(&self) -> String {  // default impl
        String::from("Reaad more...")
    }
}

pub struct NewsArticle {
    pub headline:String,
    pub author:String,
    pub content:String
}

impl Summary for NewsArticle {
}

pub struct Tweet {
    pub username:String,
    pub content:String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.content, self.username)
    }
}

pub fn traits_main() {
    let tweet = Tweet {
        username: "elon".to_string(),
        content: "Once upon a time...".to_string(),
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: "Stop the presses!".to_string(),
        author: "anonymous".to_string(),
        content: "Cuordoroy pillows, they're making headlines!".to_string(),
    };

    println!("1 new article: {}", article.summarize());
}


pub fn notify_with_display(item: &(impl Summary + Display)) {
    //
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_without_sugar<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "".to_string(),
        content: "".to_string(),
    }
}