use std::process::Command;

/**
 * ルートディレクトリでls -laを実行
 * ctrl + cで終了
 */
fn main() {
    let list_dir = Command::new("ls")
    .args(["-l", "-a"])
    .current_dir("/")  // root dir
    .spawn()  // コマンドを子プロセスとして実行し、ハンドルを返す
    .expect("failed!");
    // 表示
    list_dir.stdout;
}
