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
