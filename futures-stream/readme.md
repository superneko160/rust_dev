# Stream

RustのStreamは **非同期版のIterator** と言い換えて差し支えない。（次の要素を取得するのに `.await` が必要なIterator）類似点として、以下の特徴がある

- Iteratorと同じような操作 `map` `filter` `fold` `collect` などのメソッドが使える
- 遅延評価 必要になるまで要素を生成しない
- 順次処理 要素を1つずつ順番に処理

```rust
// Iterator: 同期的
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>  // すぐに値を返す
}

// Stream: 非同期的
trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) 
        -> Poll<Option<Self::Item>>  // Futureを返す（await可能）
}
```
