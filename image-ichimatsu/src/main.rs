use image::{Rgb, ImageBuffer};

/**
 * 市松模様の画像を作成し、保存
 */
fn main() {
    let green = Rgb::<u8>([79, 172, 135]);  // 緑
    let black = Rgb::<u8>([41, 37, 34]);  // 黒
    let size = 512;  // 縦横のサイズ
    let frame = 64;  // 1コマのサイズ
    // 市松模様を描くクロージャ
    let draw = |x, y| {
        let (xi, yi) = (x / frame, y / frame);
        match (xi % 2, yi % 2) {
            (0, 0) => green,
            (1, 0) => black,
            (0, 1) => black,
            (1, 1) => green,
            (_, _) => panic!("error"),
        }
    };
    // クロージャを指定してImageBufferを生成
    let img = ImageBuffer::from_fn(size, size, draw);
    // ファイルへ保存
    img.save("image.png").unwrap();
}