use std::io::{self, Write};
use rand::Rng;

const HAND: [&str; 3] = ["グー", "チョキ", "パー"];
const MIN_HAND: i32 = 0;
const MAX_HAND: i32 = 2;

#[derive(PartialEq, Debug)]
enum GameResult {
    Draw,
    PlayerWin,
    PlayerLose,
}

/**
 * CPUとコマンドライン上でじゃんけん
 */
fn main() {
    loop {
        match play_round() {
            GameResult::Draw => continue,
            _ => break,
        }
    }
}

/// ゲームをプレイ
fn play_round() -> GameResult {
    // プレイヤとCPUの手を取得
    let player_hand = get_player_hand();
    let cpu_hand = get_cpu_hand();
    // プレイヤとCPUの手を表示
    display_hands(player_hand, cpu_hand);
    // プレイヤとCPUの手を比較して勝敗を取得
    let result = judge_winner(player_hand, cpu_hand);
    // 勝敗を表示
    display_result(&result);

    result
}

/// プレイヤの手を取得
/// @returns i32
fn get_player_hand() -> i32 {
    loop {
        print!("じゃんけんの手を選択してください(0:グー / 1:チョキ / 2:パー):");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) if (MIN_HAND..=MAX_HAND).contains(&num) => return num,
            _ => println!("0, 1, 2 のいずれかの数値を入力してください"),
        }
    }
}

/// CPUの手を取得
/// @returns i32
fn get_cpu_hand() -> i32 {
    rand::thread_rng().gen_range(MIN_HAND..=MAX_HAND)
}

/// じゃんけんの勝敗を判定
/// @param i32 プレイヤの手
/// @param i32 CPUの手
/// @returns GameResult
fn judge_winner(player_hand: i32, cpu_hand: i32) -> GameResult {
    match (player_hand - cpu_hand + 3) % 3 {
        0 => GameResult::Draw,
        1 => GameResult::PlayerLose,
        2 => GameResult::PlayerWin,
        _ => unreachable!(),  // 理論上決して到達しないことを示す（実行時はパニック）
    }
}

/// プレイヤとCPUの手を表示
/// @param i32 player_hand プレイヤの手
/// @param i32 cpu_hand CPUの手
fn display_hands(player_hand: i32, cpu_hand: i32) {
    println!("あなたの手: {}", HAND[player_hand as usize]);
    println!("CPUの手: {}", HAND[cpu_hand as usize]);
}

/// 勝敗を表示
/// @param &GameResult
fn display_result(result: &GameResult) {
    match result {
        GameResult::Draw => println!("あいこです"),
        GameResult::PlayerWin => println!("あなたの勝ちです"),
        GameResult::PlayerLose => println!("あなたの負けです"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_judge_winner() {  // ユーザが勝利するケース
        // ユーザ：パー vs CPU：グー
        assert_eq!(GameResult::PlayerWin, judge_winner(2, 0));
        // ユーザ：グー vs CPU：チョキ
        assert_eq!(GameResult::PlayerWin, judge_winner(0, 1));
        // ユーザ：チョキ vs CPU：パー
        assert_eq!(GameResult::PlayerWin, judge_winner(1, 2));
    }

    #[test]
    fn test_lose_judge_winner() {  // ユーザが敗北するケース
        // ユーザ：チョキ vs CPU：グー
        assert_eq!(GameResult::PlayerLose, judge_winner(1, 0));
        // ユーザ：パー vs CPU：チョキ
        assert_eq!(GameResult::PlayerLose, judge_winner(2, 1));
        // ユーザ：グー vs CPU：パー
        assert_eq!(GameResult::PlayerLose, judge_winner(0, 2));
    }

    #[test]
    fn test_draw_judge_winner() {  // 引き分けのケース
        // ユーザ：グー vs CPU：グー
        assert_eq!(GameResult::Draw, judge_winner(0, 0));
        // ユーザ：チョキ vs CPU：チョキ
        assert_eq!(GameResult::Draw, judge_winner(1, 1));
        // ユーザ：パー vs CPU：パー
        assert_eq!(GameResult::Draw, judge_winner(2, 2));
    }
}
