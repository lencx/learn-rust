fn main() {
    let tweet = Tweet {
        username: String::from("lencx"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: true,
    };

    println!("I new tweet: \n[{}]", tweet.summarize());

    let new_article = NewArticle {
        headline: "Hello, Rust `trait`".to_string(),
        location: "China".to_string(),
        author: "lencx".to_string(),
        content: "A trait tells the Rust compiler about functionality a  particular type has and can share with other types.".to_string(),
    };

    println!("I new article: \n[{}]", new_article.summarize());

    notify(tweet);
    notify(new_article);
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// default
// impl Summary for NewArticle {}

impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("#{}#", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.summarize_author(), self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}
// pub fn notify(item: impl Summary) {
//     println!("Breaking news! {}", item.summarize())
// }


// fn some_fn<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
// fn some_fn<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {

// }