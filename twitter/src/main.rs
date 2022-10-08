use reqwest::Client;
use serde::Deserialize;
use url::Url;
use anyhow::Result;
use twitterapi::TwitterAPI;

mod twitterapi;

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
 */
#[tokio::main]
async fn main() {
    // 設定情報の取得
    let twitter_api = TwitterAPI::new();
    // ツイート取得
    match get_tweets(&twitter_api).await {
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
async fn get_tweets(twitter_api: &TwitterAPI) -> Result<Vec<Tweet>> {
    // パラメータ設定
    let params = vec![("screen_name", &twitter_api.screen_name)];
    // URL設定
    let url = Url::parse_with_params("https://api.twitter.com/1.1/statuses/user_timeline.json", params)?;
    let client = Client::new().get(url).header("authorization", format!("Bearer {}", &twitter_api.bearer_token));
    // ツイート取得
    let tweets: Vec<Tweet> = client.send().await?.json().await?;
    Ok(tweets)
}