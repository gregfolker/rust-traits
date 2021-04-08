// Project: rust-traits
// Author: Greg Folker

// A trait tells the Rust compiler about the functionality
// a particular type can share with other types
//
// Traits in Rust are similar to interfaces in other languages

mod lib;

use lib::*;

fn main() {
	println!("Hello, World!");

    let tweet = Tweet {
        username: String::from("horse e_books"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet:false,
    };

    println!("tweet is {}", tweet.summarize());
    println!("{}", tweet.summarize_author());

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
    println!("{}", article.summarize_author());

    // Using a trait as a parameter
    notify(&article);
}
