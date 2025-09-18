// Iteratorトレイト（イテレータ共通のインターフェース）
trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
    fn current(&self) -> Option<T>;
    fn has_next(&self) -> bool;
    fn reset(&mut self);
}

// 具象イテレータ
struct ConcreteIterator<'a, T> {
    idx: usize,  // 現在のインデックス位置
    container: &'a Container<T>,  // 参照するコンテナへの借用参照
}

// ConcreteIteratorに対するIteratorトレイトの実装
impl<'a, T: Clone> ConcreteIterator<'a, T> {
    // 新しいイテレータインスタンスの作成
    fn new(container: &'a Container<T>) -> ConcreteIterator<'a, T> {
        ConcreteIterator { idx: 0, container }
    }
}

// ConcreteIteratorに対するIteratorトレイトの実装
impl<'a, T: Clone> Iterator<T> for ConcreteIterator<'a, T> {
    // 次の要素を取得し、インデックスを1つ進める
    fn next(&mut self) -> Option<T> {
        let item = self.container.data.get(self.idx).cloned();
        self.idx += 1;
        item
    }
    // 現在の位置の要素を取得する（インデックスの変更はしない）
    fn current(&self) -> Option<T> {
        self.container.data.get(self.idx).cloned()
    }
    // 次の要素が存在するか判定
    fn has_next(&self) -> bool {
        self.container.data.len() > self.idx
    }
    // イテレータを最初の位置にリセット
    fn reset(&mut self) {
        self.idx = 0;
    }
}

// データを格納するコンテナ
struct Container<T> {
    data: Vec<T>,
}

// コンテナのメソッド
impl<T: Clone> Container<T> {
    // 新しいコンテナの作成
    fn new() -> Container<T> {
        Container { data: Vec::new() }
    }
    // アイテムの追加
    fn add_item(&mut self, item: T) {
        self.data.push(item);
    }
    // コンテナ用のイテレータを作成して返す
    // impl Iterator<T> + '_ は実装詳細を隠蔽し、ライフタイムを推論させる
    fn iter(&self) -> impl Iterator<T> + '_ {
        ConcreteIterator::new(self)
    }
}

// Client
fn main() {
    let mut names = Container::new();
    names.add_item("Alice");
    names.add_item("Bob");
    names.add_item("Eve");

    let mut iter = names.iter();

    let name1 = iter.next();
    println!("item: {:?}", name1);  // Some("Alice")

    let name2 = iter.current();
    println!("item: {:?}", name2);  // Some("Bob")

    iter.reset();
    while iter.has_next() {
        let v = iter.next().unwrap();
        println!("item: {}", v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // テスト用ののヘルパー関数
    fn create_test_container() -> Container<u8> {
        let mut container = Container::new();
        container.add_item(1);
        container.add_item(2);
        container.add_item(3);
        container
    }

    #[test]
    fn test_iterator_next_test() {
        let container = create_test_container();
        let mut iter = container.iter();
        let item = iter.next();
        assert_eq!(item, Some(1));
        let item = iter.next();
        assert_eq!(item, Some(2));
    }

    #[test]
    fn test_iterator_current_test() {
        let container = create_test_container();
        let mut iter = container.iter();
        iter.next();
        let item = iter.current();
        assert_eq!(item, Some(2));
    }

    #[test]
    fn test_iterator_has_next() {
        let container = create_test_container();
        let iter = container.iter();
        let has_next = iter.has_next();
        assert_eq!(has_next, true);
    }

    #[test]
    fn test_iterator_reset() {
        let container = create_test_container();
        let mut iter = container.iter();

        // 一旦最後までイテレート
        while iter.has_next() {
            iter.next().unwrap();
        }

        iter.reset();

        let item = iter.current();
        assert_eq!(item, Some(1));
    }

    #[test]
    fn test_iterator_next_none() {
        let container = create_test_container();
        let mut iter = container.iter();

        while iter.has_next() {
            iter.next().unwrap();
        }

        // 次の要素がない場合、None
        let item = iter.next();
        assert_eq!(item, None);
    }
}
