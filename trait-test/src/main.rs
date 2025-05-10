/**
 * Rustのトレイトは、多言語におけるインターフェースや抽象クラスに近い
 * トレイトは、ある型が実装すべきメソッドのシグネチャを定義する
 * 
 * PHPやTypeScriptのインターフェースと似ているが
 * Rustのトレイトはデフォルト実装を提供できるなど、より強力
 * 
 * impl 型名 {
 *      // メソッド
 * }
 * 
 * impl トレイト for 型名 {
 *   // メソッド
 * }
 */
pub trait Summary {
    fn summarize(&self) -> String;

    // デフォルト実装
    fn readmore(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
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
        format!(
            "{} {}",
            self.username,
            self.content
        )
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("sneko"),
        content: String::from("Hello, there."),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // デフォルト実装したメソッドを利用
    println!("{}", tweet.readmore());

    println!("=======");

    let article = NewsArticle {
        headline: String::from("S&P500横ばい、米中協議に市場身構える"),
        location: String::from("日本"),
        author: String::from("sneko"),
        content: String::from("9日の米株式市場でS&P500種株価指数は方向感の定まらない展開となり、ほぼ横ばいで取引を終えた。米国債相場も総じて小動き。世界の二大経済大国である米国と中国による貿易協議を週末に控え、投資家は積極的な取引を控えた。円相場は1ドル=145円台前半。"),
    };

    println!("New article available: {}", article.summarize());

    // デフォルト実装したメソッドを利用
    println!("{}", article.readmore());

    println!("=======");
}
