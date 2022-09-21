const OFFSET: u8 = 3; // 25以下（26以上だとアルファベットが一巡するので意味ない）

/**
 * シーザー暗号
 */
fn main() {
    let text = "AB?YZ ab*yz";  // 平文
    for c in text.chars() {
        print!("{}", encrypt(c));
    }
    println!("");
}

/*
 * 暗号化
 */
fn encrypt(c: char) -> char {
    // アルファベットでないならばそのまま返す
    if !is_alphabet(c) {
        return c;
    }
    let pos = ctrl_position(c);
    return pos as char;
}

/**
 * アルファベットか確認
 */
fn is_alphabet(c: char) -> bool {
    let c_u8 = c as u8;
    // A65 B66 .. Y89 Z90 .. a97 b98 .. y121 z122
    if (c_u8 > 64 && c_u8 < 91) || (c_u8 > 96 && c_u8 < 123) {
        return true;
    }
    false
}

/**
 * 該当する文字コードの位置調整
 */
 fn ctrl_position(c: char) -> u8 {
    let pos = c as u8;
    // A65 B66 .. Y89 Z90
    if pos > 64 && pos < 91 {
        if pos + OFFSET > 90 {
            return (pos + OFFSET) - (90 - 64);  // 90(Z)から64(A手前)を引いたぶんを差し引いて位置を調整
        }
        else {
            return pos + OFFSET;
        }
    }
    // a97 b98 .. y121 z122
    if pos > 96 && pos < 123 {
        if pos + OFFSET > 122 {
            return (pos + OFFSET) - (122 - 96);  // 122(z)から96(a手前)を引いたぶんを差し引いて位置を調整 
        }
        else {
            return pos + OFFSET;
        }
    }
    pos
 }