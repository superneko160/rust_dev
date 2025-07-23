/**
 * HaskellのfmapみたいなのをRustで書いた場合
 * main = do
 *     print (fmap (+7) (Just 10))
 *     print (fmap (+7) Nothing)
 */
fn main() {
    println!("{:?}", Some(10).map(|v| v+7));      // Some(17)
    println!("{:?}", None::<usize>.map(|v| v+7)); // None
}
