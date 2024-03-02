use std::io::{BufRead, BufReader};
use std::fs::File;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,  // コマンドライン引数 1

    #[clap(name = "FILE")]
    formula_file: Option<String>,  // コマンドライン引数 2
}

fn main() {
    let opts = Opts::parse();
    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    }
    else {
        // ファイルを指定しなかった場合
        println!("No file is specified");
    }
}