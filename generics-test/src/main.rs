fn main() {
    let t1 = make_tuple(1, 2);
    let t2 = make_tuple("Hello", "world");
    let t3 = make_tuple(vec![1, 2, 3], vec![4, 5]);
    let t4 = make_tuple(3, "years old");
    println!("t1: {:?}", t1);
    println!("t2: {:?}", t2);
    println!("t3: {:?}", t3);
    println!("t4: {:?}", t4);
}

fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    (t, s)
}

// テスト用のモジュールを定義
#[cfg(test)]
mod tests {
    #[test]
    fn test_make_tuple() {
        let t1 = super::make_tuple(1, 2);
        assert_eq!(t1, (1, 2));

        let t2 = super::make_tuple("Hello", "world");
        assert_eq!(t2, ("Hello", "world"));

        let t3 = super::make_tuple(vec![1, 2, 3], vec![4, 5]);
        assert_eq!(t3, (vec![1, 2, 3], vec![4, 5]));

        let t4 = super::make_tuple(3, "years old");
        assert_eq!(t4, (3, "years old"));
    }
}