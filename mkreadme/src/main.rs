use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

/* 
 * コマンドライン引数に指定されたディレクトリ下にテンプレートのreadmeファイルを作成
 * cargo run /workspaces/rust_dev (Dockerfile環境下の場合)
 */
fn main() {
    // コマンドライン引数取得
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    // 絶対パス作成
    let path = path.to_string() + "/readme.md";  // String + &str
    let path = Path::new(&path);
    let display = path.display();
    // ファイル作成（ファイルを書き込み専用で開く。すでに存在している場合、破棄して新しく作成）
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",display, why),
        Ok(file) => file,
    };
    // ファイル書き込み
    let template_str = "# README\n## Versions\n## Setting\n## Run\n## Reference\n";
    match file.write_all(&template_str.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}