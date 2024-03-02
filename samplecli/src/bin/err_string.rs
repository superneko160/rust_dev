// bin/err_panic.rsを正常終了時はi32型、エラー時はString型を返すように修正したもの
// Ok(24)
// Err(invalid digit found in string)
fn get_int_from_file() -> Result<i32, String> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| e.to_string())
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}