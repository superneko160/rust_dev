use reqwest::Client;
use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize)]
struct Forecast {
    latitude: f32,
    longitude: f32,
    generationtime_ms: f32,
    utc_offset_seconds: i32,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f32,
    daily_units: DailyUtils,
    daily: Daily,
}

#[derive(Debug, Deserialize)]
struct DailyUtils{
    time: String,
    temperature_2m_max: String,
    temperature_2m_min: String,
    weathercode: String,
}

#[derive(Debug, Deserialize)]
struct Daily {
    time: Vec<String>,
    temperature_2m_max: Vec<f32>,
    temperature_2m_min: Vec<f32>,
    weathercode: Vec<f32>,
}

/**
 * Open-MeteoのWebAPIを呼び出して東京都の最高気温、最低気温、WMO気象コードを取得
 */
#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let url = "https://api.open-meteo.com/v1/forecast";
    let params = [
        ("latitude", "35.6785"),  // 緯度
        ("longitude", "139.6823"),  // 経度
        ("daily", "temperature_2m_max"),  // 最高気温
        ("daily", "temperature_2m_min"),  // 最低気温
        ("daily", "weathercode"),  // WMO気象コード
        ("timezone", "Asia/Tokyo")  // タイムゾーン
    ];
    let response = client.get(url).query(&params).send().await?;
    let forecast = response.json::<Forecast>().await?;
    for (i, date) in forecast.daily.time.iter().enumerate() {
        println!("日付：{} 最高気温：{} 最低気温：{} 気象コード：{}",
            date,
            forecast.daily.temperature_2m_max[i],
            forecast.daily.temperature_2m_min[i],
            forecast.daily.weathercode[i]
        );
    }
    Ok(())
}

/*
 * 型を表示
 */
// fn p_typeof<T>(_: T) {
//     println!("{}", std::any::type_name::<T>());
// }