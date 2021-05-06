use trait_sample01::Tweet;
use trait_sample01::Summary;
use trait_sample01::NewsArticle;
use std::fmt::Display;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());

    // トレイト境界構文を使用した例
    //notify(&tweet);

    let tweet = returns_summarizable();
    println!("{}", tweet.summarize());
}

// トレイト境界構文
//pub fn notify(item: &impl Summary) {
//    println!("Breaking news! {}", item.summarize());
//}

// これは上のメソッドの糖衣構文(syntax sugar)
//pub fn notify<T: Summary>(item: &T) {
//    println!("Breaking news! {}", item.summarize());
//}

//pub fn notify(item: &(impl Summary + Display)) {
//    println!("Breaking news! {}", item.summarize());
//}

//pub fn notify<T: Summary + Display>(item: &T) {
//    println!("Breaking news! {}", item.summarize());
//}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("house_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}