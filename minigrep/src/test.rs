use super::*;

#[test]
fn one_result() {
    let query: &str = "duct";
    let contents: &str = "\
Rust:
safe, fast, productive.
Pick there.";

    // 正常系：productiveのなかにductが入っている
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}
