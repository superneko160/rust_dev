use reqwest::Client;
use serde::Deserialize;
use url::Url;

// 本当は環境変数に組み込む
const SCREEN_NAME: &str = "bot_kannachan";  // スクリーンネーム（@hogehoge → hogehoge）
const BEARER_TOKEN: &str = "";  // ベアラートークン

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// response: https://developer.twitter.com/en/docs/twitter-api/v1/tweets/timelines/api-reference/get-statuses-user_timeline
#[derive(Deserialize, Debug)]
struct Tweet {
  created_at: String,
  id: u64,
  text: String,
}

/**
 * 特定のユーザの直近のツイート取得
 * open-ssl系のエラーが出た場合、apt update と apt install libssl-dev
 */
#[tokio::main]
async fn main() -> Result<()> {
    let params = vec![("screen_name", SCREEN_NAME)];
    let url = Url::parse_with_params("https://api.twitter.com/1.1/statuses/user_timeline.json", params)?;
    let client = Client::new().get(url).header("authorization", format!("Bearer {}", BEARER_TOKEN));
    let tweets: Vec<Tweet> = client.send().await?.json().await?;
    for tweet in tweets {
        println!("{} {} {}", tweet.created_at, tweet.id, tweet.text);
    }
    Ok(())
}