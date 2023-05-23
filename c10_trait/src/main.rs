use c10_trait::{Summary, Tweet, NewsArticle, notify};
use c10_trait::pair;

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Donald Trump"),
        content: String::from(
            "MAGA Make America Great Again",
        ),
        reply: false,
        retweet: false,
    }
}


use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);

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
    println!("{}", article.content);

    println!("{}", returns_summarizable().summarize());

    let pair_obj = pair::Pair::new(5, 12);
    pair_obj.cmp_display();
    println!("pair ojbect is {}.", pair_obj);
    println!("pair ojbect is {}.", pair_obj.to_string());


    println!("{}", longest_with_an_announcement("ABCDEF", "ABCD", article));

}