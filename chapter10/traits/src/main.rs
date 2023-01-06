use std::fmt::Display;

fn main() {
    using_the_trait_methods();
}

// Section: implementing trait for types thay implements other traits
impl<T: Display> ToString for T {  // the standard library implements ToString only for types that implements Display
                                   // this way, we can call to_string for types like i32 that implements Display
    // --snip--
}

// Section: using trait bounds to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {  // new() is implemented for any type T
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {  // cmp_display() is implemented only for types that implements the traits Display and PartialOrd
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Section: returning types that implements traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// fn returns_summarizable(switch: bool) -> impl Summary {  // The return type can be anything that implements Summary, but only one type
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }


// Section: clearer trait bounds with where clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

}

fn some_function_v2<T, U>(t: &T, u: &U) -> i32 // clearer version
where
    T: Display + Clone,
    U: Clone + Debug,
{

}

// Section: multiple trait bounds with the + Syntax
fn notify_v3(item: &(impl Summary + Display)) {

}

fn notify_v4<T: Summary + Display>(item: &T) {

}

// Section: multiple parameters with trait bounds

fn collect(item1: &impl Summary, item2: &impl Summary) {
    // With this signature, item1 and item2 can be of DIFFERENT types, as long as
    // both implement Summary
}

fn collect_v2<T: Summary>(item1: &T, item2: &T) {
    // With this signature, item1 and item2 MUST be of the same type
}

// Section: trait as parameters (trait bounds)
fn notify(item: &impl Summary) {  // the parameter is an immutable reference to any type that implements Summary
    println!("Breaking news! {}", item.summarize());
}

fn notify_v2<T: Summary>(item: &T) {  // trait bounds uses generics, but this syntax is equivalent to the previous one
    println!("Breaking news! {}", item.summarize());
}

// Section: trait basics

fn using_the_trait_methods() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let book = Book {
        title: String::from("Grande Sertão: Veredas"),
        author: String::from("João Guimarães Rosa"),
        year: String::from("2006"),
        publisher: String::from("Nova Fronteira"),
    };

    println!("New book added: {}", book.summarize());
}

pub trait Summary {
    // fn summarize(&self) -> String;  // without default implementation
    
    // with default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
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
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct Book {
    pub title: String,
    pub author: String,
    pub year: String,
    pub publisher: String,
}

impl Summary for Book {  // uses default implementation of summarize
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}
