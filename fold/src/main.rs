// リストの合計値を算出
fn listsum(numbers: Vec<u32>) -> u32 {
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    sum
}

// 文字列の連結
fn concat(words: Vec<&str>) -> String {
    let sentence = words.iter().fold(String::new(), |mut acc, &word| {
        if !acc.is_empty() {
            acc.push(' ');  // 最初の単語以外は前にスペース追加
        }
        acc.push_str(word);
        acc
    });
    sentence
}

// 最大値を見つける
fn find_max(numbers: Vec<u32>) -> Option<u32> {
    let max_val = numbers.iter().fold(None, |acc: Option<u32>, &x| {
        match acc {
            Some(current_max) => Some(current_max.max(x)),
            None => Some(x),  // accが初期状態（None）なら、xが最初の最大値
        }
    });
    max_val
}

// 文字列のリストをCSV形式に変更
fn create_csv_string(items: Vec<&str>) -> String {
    let csv_string = items.iter().fold(String::new(), |mut acc, &item| {
        if !acc.is_empty() {
            acc.push(',');  // 最初の単語以外は前にカンマ追加
        }
        acc.push_str(item);
        acc
    });
    csv_string
}

fn main() {
    let nums = [70, 25, 5];
    println!("sum = {}", listsum(nums.to_vec()));

    let words = vec!["Rust", "is", "awesome"];
    println!("{}", concat(words));

    println!("{:?}", find_max(nums.to_vec()));

    let fruits = vec!["apple", "banana", "cherry"];
    println!("{}", create_csv_string(fruits));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_csv_string_basic() {
        let items = vec!["apple", "orange", "grape"];
        let expected = "apple,orange,grape";
        let result = create_csv_string(items);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_create_csv_string_single_item() {
        let items = vec!["only_item"];
        let expected = "only_item";
        let result = create_csv_string(items);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_create_csv_string_empty() {
        let items: Vec<&str> = Vec::new();
        let expected = "";
        let result = create_csv_string(items);
        assert_eq!(result, expected);
    }
}
