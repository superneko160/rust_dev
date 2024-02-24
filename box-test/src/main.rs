/**
 * Boxを使うと値はヒープ領域に確保される
 * ヒープ領域に確保する値は、コンパイル時にサイズがわかっていなくても問題ない
 */
fn main() {
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    print(Box::new(byte_array));
    let byte_array = [b'h', b'e', b'l', b'l', b'o', b'!'];
    print(Box::new(byte_array));
}

fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}
