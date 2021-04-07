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
}
