fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![0; 5];  // 0を5つ埋めて初期化
    println!("{:?}", v1[2]);  // 3
    for val in &v2 {
        println!("{:?}", val);
    }
}
