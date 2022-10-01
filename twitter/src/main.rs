use std::env;
use reqwest::Client;
use serde::Deserialize;
use url::Url;
use anyhow::Result;

// response: https://developer.twitter.com/en/docs/twitter-api/v1/tweets/timelines/api-reference/get-statuses-user_timeline
#[derive(Deserialize, Debug)]
struct Tweet {
  created_at: String,
  text: String,
}

/**
 * 特定のユーザの直近のツイート取得
 * 環境変数設定: export SCREEN_NAME="xxxxx" + export BEARER_TOKEN="xxxxx"
 * 環境変数確認: printenv
 * open-ssl系のエラーが出た場合、apt update + apt install libssl-dev + apt install pkg-config
 */
#[tokio::main]
async fn main() {
    // ツイート取得
    match get_tweets().await {
        Ok(tweets) => {
            // ツイート表示
            for tweet in tweets {
                println!("{} {}", tweet.created_at, tweet.text);
            }
        },
        Err(e) => {
            // エラー表示
            eprintln!("{}", e);
        }
    }
}

/**
 * ツイート取得
 */
async fn get_tweets() -> Result<Vec<Tweet>> {
    // 環境変数取得
    let screen_name = env::var("SCREEN_NAME").expect("SCREEN_NAME is not defined");
    let bearer_token = env::var("BEARER_TOKEN").expect("BEARER_TOKEN is not defined");
    // パラメータ設定
    let params = vec![("screen_name", screen_name)];
    // URL設定
    let url = Url::parse_with_params("https://api.twitter.com/1.1/statuses/user_timeline.json", params)?;
    let client = Client::new().get(url).header("authorization", format!("Bearer {}", bearer_token));
    // ツイート取得
    let tweets: Vec<Tweet> = client.send().await?.json().await?;
    Ok(tweets)
}