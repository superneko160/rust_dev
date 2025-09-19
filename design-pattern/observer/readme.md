# Observerパターン

あるオブジェクト(Subject)の状態が変化したとき、それを監視している複数のオブジェクト(Observer)に自動で通知を行う仕組みを提供する

## 構成要素

### Subject（被観察者）

- 状態を持ち、状態の変更時、Observerに通知を行う
- インターフェース

### Observer（観察者）

- 状態の変化に応じた処理を実装
- インターフェース

### ConcreteSubject

- 実際のステータスを管理する
- Subjectを実装したクラス

### ConcreteObserver

- 通知を受けて処理を行うクラス
- Observerを実装したクラス

## 補足

### `dyn` に関して

- `dyn` は動的ディスパッチを実現するためのキーワード
- `dyn` は dynamic の略でトレイトオブジェクトの作成時に使用する
- これにより異なる具象型を同じトレイトを通して統一的に利用できる

```rs
// 静的ディスパッチ（Generic）
// コンパイル時に具体的な型が決まる
fn process_status<T: DeliveryStatus>(status: &T) {
    println!("{}", status);
}

// 実際には以下のような関数が生成される
fn process_status_getting_ready(status: &GettingReady) { ... }
fn process_status_delivered(status: &Delivered) { ... }
```

```rs
// 動的ディスパッチ（dyn）
// 実行時に具体的な型が決まる
fn process_status(status: &dyn DeliveryStatus) {
    println!("{}", status);
}

// 1つの関数で様々な型を処理
```

`dyn` を使うユースケースとして以下の場面が挙げられる。以下のコードは、異なる型を同じコレクションに格納したい

```rs
// error!（異なる型を同じVecに入れられない）
let statues = vec![
    GettingReady,  // GettingReady型
    Delivered,     // Delivered型
];
```

```rs
// トレイトオブジェクトとして統一的に扱える
let statuses: Vec<Box<dyn DeliveryStatus>> = vec![
    Box::new(GettingReady),
    Box::new(Delivered),
];
```

サンプルコード内での `dyn` の使用例

```rs
// 異なるObserver実装を同じVecに格納
pub struct DeliveryTracker {
    observers: Vec<Box<dyn DeliveryObserver>>,  // dyn使用
}

// 異なる型のObserverを統一的に扱える
delivery_tracker.add_observer(Box::new(AppNotifier));    // AppNotifier型
delivery_tracker.add_observer(Box::new(EmailNotifier));  // EmailNotifier型
```

```rs
impl DeliveryObserver for AppNotifier {
    fn update(&self, status: &dyn DeliveryStatus) {  // dyn使用
        // どの具象型のDeliveryStatusでも受け取れる
    }
}
```

`dyn` を使わない場合、以下のような制約がある

```rs
// ❌ 以下は不可能（異なる型を同じ関数で処理できない）
fn notify_all_generic(
    app: &AppNotifier, 
    email: &EmailNotifier, 
    status_ready: &GettingReady,
    status_delivered: &Delivered
) {
    // 各組み合わせごとに別々の処理が必要になる
}

// ✅ dynを使えば統一的に処理可能
fn notify_all_dynamic(
    observers: &[Box<dyn DeliveryObserver>],
    status: &dyn DeliveryStatus
) {
    for observer in observers {
        observer.update(status);  // 統一的に処理
    }
}
```
