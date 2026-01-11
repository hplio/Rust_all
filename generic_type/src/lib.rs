pub trait Summury {
    fn summarize(&self) -> String;
}

pub struct NewsArtical {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summury for NewsArtical {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
// pub fn notify<T: Summury>(item: &T) {
//     println!("Bracking news: {}", item.summarize());
// }
pub fn notify(item: &impl Summury, item1: &impl Summury) {
    println!("Bracking news: {}", item.summarize());
    print!("\n");
    println!("Bracking news: {}", item1.summarize());
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub replay: bool,
    pub repost: bool,
}

impl Summury for SocialPost {
    fn summarize(&self) -> String {
        format!(
            "content: {} and it's username is:{}",
            self.content, self.username
        )
    }
}
