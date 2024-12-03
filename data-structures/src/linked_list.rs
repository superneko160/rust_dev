// リストの各要素
struct Node {
    value: i32,  // 要素の値
    next: Option<Box<Node>>,  // 次の要素への参照
}

// 単方向リスト
pub struct LinkedList {
    // リストの中身を保存する要素
    head: Option<Box<Node>>,
}

impl LinkedList {
    // 新しいリストを作成
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // 先頭に要素を追加
    pub fn add(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),  // 現在の先頭を新しいノードの次の要素にする
        });
        self.head = Some(new_node);
    }

    // リストの内容を表示
    pub fn print(&self) {
        let mut current = &self.head;
        
        while let Some(node) = current {
            // 現在のノードの値を表示
            print!("{} ", node.value);
            
            // 次のノードに進む
            current = &node.next;
        }
        println!(); // 改行
    }

    // 特定の値を探す
    pub fn find(&self, target: i32) -> Option<i32> {
        let mut current = &self.head;

        while let Some(node) = current {
            if node.value == target {
                return Some(node.value);  // 値が見つかった
            }

            // 次のノードに進む
            current = &node.next;
        }

        None  // 値が見つからなかった
    }

    // 先頭の要素を返す
    pub fn first(&self) -> Option<i32> {
        self.head.as_ref().map(|node| node.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list_is_empty() {
        let list = LinkedList::new();
        assert_eq!(list.first(), None);
    }

    #[test]
    fn test_add_single_element() {
        let mut list = LinkedList::new();
        list.add(32);
        assert_eq!(list.first(), Some(32));
        assert_eq!(list.find(32), Some(32));
    }

    #[test]
    fn test_add_multiple_elements() {
        let mut list = LinkedList::new();
        list.add(10);
        list.add(20);
        list.add(30);

        assert_eq!(list.find(10), Some(10));
        assert_eq!(list.find(20), Some(20));
        assert_eq!(list.find(30), Some(30));
    }

    #[test]
    fn test_find_non_existent_element() {
        let mut list = LinkedList::new();
        list.add(42);
        assert!(!list.find(100).is_some());
    }

    #[test]
    fn test_add_and_find_multiple_times() {
        let mut list = LinkedList::new();
        
        let test_values = [1, 5, 10, 15, 20];
        for &value in &test_values {
            list.add(value);
        }

        for &value in &test_values {
            assert!(list.find(value).is_some(), "値 {} が見つからない", value);
        }
    }
}
