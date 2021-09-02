pub trait Summary {
    fn summarize(&self) -> String;
    fn read_more(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn summarizer(obj: &impl Summary) {
    println!("[Using summarizer function]: {}", obj.summarize());
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/**
 * pub fn notify(item1: &impl Summary, item2: &impl Summary) {
 * pub fn notify<T: Summary>(item1: &T, item2: &T) {
 *
 *  Specifying Multiple Trait Bounds with the + Syntax
 * pub fn notify(item: &(impl Summary + Display)) {
 * pub fn notify<T: Summary + Display>(item: &T) {
 *
 *  Clearer Trait Bounds with where Clauses
 * fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
 * fn some_function<T, U>(t: &T, u: &U) -> i32
 *      where T: Display + Clone,
 *      U: Clone + Debug
 * {
 **/

// Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn run() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    println!("{}", article.read_more());
    println!("{}", tweet.read_more());
    summarizer(&article);
    summarizer(&tweet);
    notify(&article);
    notify(&tweet);
    let t = returns_summarizable();
    summarizer(&t);

    let pair = Pair::new(5, 6);
    pair.cmp_display()
}
