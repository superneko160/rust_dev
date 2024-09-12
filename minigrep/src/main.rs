use std::env;
use std::process;
use minigrep::Config;

/// ファイルから特定の文字列を利用している行を抽出
/// コマンドライン引数で第1引数に対象となる文字、第2引数に対象となるファイルを指定
/// ex) cargo run frog poem.txt
fn main() {
    let args = env::args();

    let config: Config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
