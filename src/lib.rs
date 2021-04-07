pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Defining a `trait`
pub trait Summary {
    // Declaration of method signatures that describe
    // the behavior of the types that implement this trait
    fn summarize(&self) -> String;
}

// The actual implementation of the trait we defined as `Summary` for the type, `NewsArticle`
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {} by ({})", self.headline, self.author, self.location)
    }
}

//  Another implementation of the trait we defined as `Summary` for a different type, `Tweet`
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
