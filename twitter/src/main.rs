use reqwest::Client;

const USER_NAME: &str = "";  // 正確にはスクリーンネームと呼ばれるもの（@hogehoge）
const BEARER_TOKEN: &str = "";  // 本当は環境変数に組み込む
/**
 * open-ssl系のエラーが出た場合、apt install libssl-dev
 */
#[tokio::main]
async fn main() {
    let url = format!("https://api.twitter.com/1.1/statuses/user_timeline.json?screen_name={}", USER_NAME);
    let token = format!("Bearer {}", BEARER_TOKEN);
    let client = Client::new().get(&url).header("authorization", token);
    let res = client.send().await.unwrap().text().await.unwrap();
    println!("{}", res);
}