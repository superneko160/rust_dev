pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {

    // 新しいスタックを作成
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    // 要素を末尾に追加
    pub fn push(&mut self, elem: T) {
        self.items.push(elem);
    }

    // 要素を末尾から取得
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // スタックが空か判定
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // スタックの長さを取得
    pub fn len(&self) -> usize {
        self.items.len()
    }

    // 先頭の要素を参照
    pub fn peek(&mut self) -> Option<&T> {
        self.items.first()
    }
}

#[cfg(test)]
mod test_stack {
    use super::*;

    #[test]
    fn stack_operations() {
        let mut stack = Stack::new();
        
        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.len(), 3);

        assert_eq!(stack.peek(), Some(&1));

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
    }
}
