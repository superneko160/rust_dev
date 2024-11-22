pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {

    // 新しいキューを作成
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    // 要素を末尾に追加
    pub fn enqueue(&mut self, elem: T) {
        self.items.push(elem);
    }

    // 先頭から要素を取得
    pub fn dequeue(&mut self) -> Option<T> {
        if !self.items.is_empty() {
            Some(self.items.remove(0))
        } else {
            None
        }
    }

    // キューが空か判定
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // キューの長さを取得
    pub fn len(&self) -> usize {
        self.items.len()
    }

    // 先頭の要素を参照
    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }
}

#[cfg(test)]
mod test_queue {
    use super::*;

    #[test]
    fn queue_operations() {
        let mut queue = Queue::new();

        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.peek(), None);

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.len(), 3);

        assert_eq!(queue.peek(), Some(&1));

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert!(queue.is_empty());
    }
}
