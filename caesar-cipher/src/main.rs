/**
 * シーザー暗号
 */
fn main() {
    // let mut offset: u8 = 0;
    // 平文
    // FABER EST SUAE QUISQUE FORTUNAE APPIUS CLAUDIUS CAECUS DICTUM ARCANUM EST NEUTRON
    let text = "MHILY LZA ZBHL XBPZXBL MVYABUHL HWWPBZ JSHBKPBZ JHLJBZ KPJABT HYJHUBT LZA ULBAYVU";
    for i in 1..26 {
        let offset: u8 = i;
        print!("{}: ", i);
        for c in text.chars() {
            print!("{}", encrypt(c, offset));
        }
        println!("");
    }
}

/*
 * 暗号化
 */
fn encrypt(c: char, offset: u8) -> char {
    // アルファベットでないならばそのまま返す
    if !is_alphabet(c) {
        return c;
    }
    let pos = ctrl_position(c, offset);
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
 fn ctrl_position(c: char, offset: u8) -> u8 {
    let pos = c as u8;
    // A65 B66 .. Y89 Z90
    if pos > 64 && pos < 91 {
        if pos + offset > 90 {
            return (pos + offset) - (90 - 64);  // 90(Z)から64(A手前)を引いたぶんを差し引いて位置を調整
        }
        else {
            return pos + offset;
        }
    }
    // a97 b98 .. y121 z122
    if pos > 96 && pos < 123 {
        if pos + offset > 122 {
            return (pos + offset) - (122 - 96);  // 122(z)から96(a手前)を引いたぶんを差し引いて位置を調整 
        }
        else {
            return pos + offset;
        }
    }
    pos
 }

#[test]
fn test_is_alphabet_true() {
    let c: char = 'a';
    assert_eq!(is_alphabet(c), true);
}

#[test]
fn test_is_alphabet_false() {
    let c: char = '-';
    assert_eq!(is_alphabet(c), false);
}

#[test]
fn test_encrypt() {
    let pre_c: char = 'a';
    let result_c: char = 'd';
    assert_eq!(encrypt(pre_c, 3), result_c);
}
