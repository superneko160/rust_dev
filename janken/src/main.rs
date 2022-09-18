use std::io;
use rand::Rng;

const HAND: [&str; 3] = ["グー", "チョキ", "パー"];

/**
 * CPUとコマンドライン上でじゃんけん
 */
fn main() {
    loop {
        println!("じゃんけんの手を選択してください(0:グー,1:チョキ,2:パー)");
        let mut player_hand = String::new();
        // 入力受付
        io::stdin().read_line(&mut player_hand).expect("入力受付に失敗");
        // 数値に変換
        let player_hand: i32 = match player_hand.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(" 数値を入力してください");
                continue;
            }
        };
        if player_hand == 0 || player_hand == 1 || player_hand == 2 {
            // CPUの手をランダムで取得
            let cpu_hand: i32 = rand::thread_rng().gen_range(0..3);
            // プレイヤーとCPUの手を表示
            println!("あなたの手:{}", HAND[player_hand as usize]);
            println!("CPUの手:{}", HAND[cpu_hand as usize]);
            // 勝者を判定しループ脱出（あいこならば継続）
            let result: i32 = player_hand - cpu_hand;
            if result == 0 {
                println!("あいこです");
                continue;
            }
            judge_winner(result);
            break;
        }
    }
}

/**
 *じゃんけんの勝敗を判定
 */
fn judge_winner(result: i32) {
    if result == 2 || result == -1 {
        println!("あなたの勝ちです");
    }
    if result == 1 || result == -2 {
        println!("あなたの負けです");
    }
}