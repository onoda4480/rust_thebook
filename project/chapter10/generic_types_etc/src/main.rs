use generic_types_etc::{Summary, NewsArticle, Tweet};

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

}
//✅ デフォルト実装 → impl で書かなくていい
//✅ tweet.summarize() が使える理由:
//    - tweet は Tweet のインスタンス
//    - Tweet は Summary を実装している
//    - だから Summary のメソッドが使える