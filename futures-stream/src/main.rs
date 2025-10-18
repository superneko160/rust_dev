use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    {
        println!("1. 基本的なStream:");
        let mut stream = stream::iter(0..5);
        while let Some(value) = stream.next().await {
            print!("{} ", value);
        }
        print!("\n");
    }

    {
        println!("\n2. mapで各要素を2倍に:");
        let stream = stream::iter(1..=5).map(|x| x * 2);
        let results: Vec<i32> = stream.collect().await;
        println!("  結果: {:?}", results);
    }

    {
        println!("\n3. filterで偶数のみ抽出:");
        let stream = stream::iter(1..=10).filter(|&x| async move { x % 2 == 0 });
        let evens: Vec<i32> = stream.collect().await;
        println!("  偶数: {:?}", evens);
    }

    {
        println!("\n4. foldで合計を計算:");
        let sum = stream::iter(1..=5).fold(0, |acc, x| async move { acc + x }).await;
        println!("  1~5の合計: {}", sum);
    }
}
