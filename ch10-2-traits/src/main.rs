extern crate aggregator;

use aggregator::NewsArticle;
use aggregator::NewsArticleWoSummary;
use aggregator::Summarizable;
use aggregator::Tweet;

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
    author: String,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
                 precipitation is {}%.", self.high_temp, self.low_temp,
                 self.chance_of_precipitation)
    }
    fn author_summary(&self) -> String {
        format!("{}", self.author)
    }
}

pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summary());

    notify(tweet);

    let forecast = WeatherForecast {
        high_temp: 25.0,
        low_temp: 18.0,
        chance_of_precipitation: 33.0,
        author: String::from("forecast"),
    };
    println!("Weather forecast: {}", forecast.summary());

    notify(forecast);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    println!("New article available! {}", article.summary());

    let article = NewsArticleWoSummary {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    println!("New article available! {}", article.summary());
}