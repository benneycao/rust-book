use std::thread::sleep;

/// this trait have only method to summary you self and return a sentence
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: String,
    retweet: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("This is NewsArticle summarize, and author is {:?}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("This is Tweet summarize, and username is {:?}, content is {:?}", self.username, self.content)
    }
}

fn notify(item: impl Summary) {
    println!("Breaking news {}", item.summarize());
}

#[test]
fn test_summary_trait() {
    let tweet = Tweet {
        username: "cjx_tweet".to_string(),
        content: "my name is cjx".to_string(),
        reply: "".to_string(),
        retweet: "".to_string(),
    };
    let news_article = NewsArticle {
        headline: "".to_string(),
        location: "Beijing".to_string(),
        author: "Benny cao at News".to_string(),
        content: "my name is Benny cao".to_string(),
    };

    eprintln!("tweet = {}", tweet.summarize());
    eprintln!("news = {}", news_article.summarize());

    notify(tweet);
    notify(news_article);
}










