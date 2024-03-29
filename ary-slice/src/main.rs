fn main() {
    // 配列を参照するときは自動的にスライスとして扱われる
    // スライスとして扱えると[start..end]と範囲指定できるので便利
    let a: [i32; 3] = [0, 1, 2];
    println!("{:?}", &a[1..3]);  // [1, 2]
    let b: [i32; 3] = [0; 3];  // 0を3つ埋めて初期化
    println!("{:?}", &b[0..3]);  // [0, 0, 0]
}
