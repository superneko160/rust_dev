# `From` と `Into`

## `From` トレイト

- 目的: ある型から別の型への変換
- 実装: 変換先の型（この場合Point）に対して実装
- メソッド: `from(input: T) -> Self`

```rs
// f64型の値からPoint型のインスタンスを作成
impl From<f64> for Point {
    fn from(input: f64) -> Self {
        Point { x: input, y: input }
    }
}
```

## `Into` トレイト

- 特徴: `From<T> for U` を実装すると `Into<U> for T` が自動的に実装される
- 使い方: 変換元の値に対して `.into()` メソッドを呼び出す
- 型推論: 変換先の型を明示的に指定する必要がある

```rs
let p1 = Point::from(1.0);     // Fromトレイトを直接使用
let p2: Point = (1.0).into();  // Intoトレイトを使用（型注釈が必要）
```

## `From` と `Into` の違い

サンプルコードだと両方とも `Point { x: 1.0, y: 1.0 }` と同じ結果になる。2つの違いは、呼び出し方法と設計思想にある

### 1. 呼び出し方法

```rs
let p1 = Point::from(1.0);     // 関連関数として呼び出し
let p2: Point = (1.0).into();  // メソッドとして呼び出し
```

- `From::from()` : 静的な関連関数（ `Type::function()` 形式）
- `.into()` : インスタンスメソッド（ `value.method()` 形式）

## 2. 設計思想

`From` : 「この型から作る」という構築的な視点
`Into` : 「この値を変換する」という変換的な視点

```rs
// 明示的な型変換が目的なら From
let point1 = Point::from(1.0);

// 既存の値を別の型に変換するなら Into
let x = 2.0;
let point2: Point = x.into();

// ジェネリック境界では Into がよく使われる
fn draw_point<P: Into<Point>>(p: P) {
    let point = p.into();
    // 描画処理
}
```
