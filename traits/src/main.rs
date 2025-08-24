pub struct SocialPost {
    pub username: String,
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let article = SocialPost {
        username: String::from("aa"),
    };

    println!("New article available! {}", article.summarize());
}
