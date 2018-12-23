mod lib;
use crate::lib::Summary;
use crate::lib::Tweet;
use crate::lib::NewsArticle;

fn main() {
    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {:?}", tweet.summarize());
    }

    {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Chanpionship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
                                  hockey team in the NHL."),
        };

        println!("New article available! {}", article.summarize());
    }
}
