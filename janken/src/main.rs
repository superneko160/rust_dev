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

        // 入力された手を数値に変換
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

            // 勝者を判定しループ離脱（あいこならば継続）
            match judge_winner(player_hand - cpu_hand) {
                1 | 2 => break,
                _ => continue
            }
        }
    }
}

/// じゃんけんの勝敗を判定
/// @param i32 ユーザの手 - CPUの手（2 or -1:ユーザの勝利、1 or -2:CPUの勝利、0:あいこ）
/// @returns i8 0:あいこ、1:ユーザの勝利、2:ユーザの敗北
/// 
fn judge_winner(result: i32) -> i8 {
    if result == 2 || result == -1 {
        println!("あなたの勝ちです");
        return 1;
    }
    if result == 1 || result == -2 {
        println!("あなたの負けです");
        return 2;
    }
    println!("あいこです");
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_judge_winner() {  // ユーザが勝利するケース
        let win_result: i8 = 1;
        // ユーザ：パー vs CPU：グー
        assert_eq!(win_result, judge_winner(2 - 0));
        // ユーザ：グー vs CPU：チョキ
        assert_eq!(win_result, judge_winner(0 - 1));
        // ユーザ：チョキ vs CPU：パー
        assert_eq!(win_result, judge_winner(1 - 2));
    }

    #[test]
    fn test_lose_judge_winner() {  // ユーザが敗北するケース
        let win_result: i8 = 2;
        // ユーザ：チョキ vs CPU：グー
        assert_eq!(win_result, judge_winner(1 - 0));
        // ユーザ：パー vs CPU：チョキ
        assert_eq!(win_result, judge_winner(2 - 1));
        // ユーザ：グー vs CPU：パー
        assert_eq!(win_result, judge_winner(0 - 2));
    }

    #[test]
    fn test_draw_judge_winner() {  // 引き分けのケース
        let win_result: i8 = 0;
        // ユーザ：グー vs CPU：グー
        assert_eq!(win_result, judge_winner(0 - 0));
        // ユーザ：チョキ vs CPU：チョキ
        assert_eq!(win_result, judge_winner(1 - 1));
        // ユーザ：パー vs CPU：パー
        assert_eq!(win_result, judge_winner(2 - 2));
    }
}