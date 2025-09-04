# ステートパターン

- オブジェクトの内部状態に応じて振る舞いを変える
- 条件分岐を多様することなく、振る舞いをオブジェクト自体（状態クラス）にカプセル化できる
- これにより、状態ごとのロジックが独立したクラスに分けられる

## 構成要素

### 1.振る舞いを定義するトレイト（ `State` ）

- `State` トレイトは、投稿が持ちうるすべての状態（下書き、レビュー待ち、公開済みなど）が共通して持つべき振る舞いを定義する。ここでは `request_review` `approve` `content` の3つのメソッドのこと
- これらのメソッドは、オブジェクトが現在どの状態にあるかによって、異なる動作をする

### 2. 具体的な状態を表現する構造体（ `Draft` `PendingReview` `Published` ）

- これらの構造体は `State` トレイトを実装し、投稿の具体的な状態ごとのロジックをカプセル化している
- `Draft` （下書き）:
    - `request_review` が呼ばれると、状態を `PendingReview` に遷移する
    - `approve` が呼ばれても自身を返すため、下書き状態からいきなり公開されることはない
    - `content` メソッドは空文字列を返す。これは、下書きの内容はまだ公開可能ではないことを意味する
- `PendingReview` （レビュー待ち）
    - `request_review` が呼ばれても状態は変更されない
    - `approve` が呼ばれると、状態を `Published` に遷移させる
- `Published` （公開済み）
    - `request_review` or `approve` が呼ばれても状態の変更はない
    - `content` メソッドは、投稿の実際のコンテンツを返す

### 3. 状態を管理するコンテキスト（ `Post` ）

- `Post` 構造体は、投稿のコンテキスト。現在の状態オブジェクト（ `Box<dyn State>` ）への参照を保持する
- `request_review` や`approve` が呼ばれると、 `Post` オブジェクトは直接ロジックを実行するのではなく、現在の状態オブジェクトに処理を移譲する
- `self.state.take()` と `self.state = Some(...)` を使って、状態オブジェクトの所有権を取り出し、新しい状態オブジェクトに置き換えることで、状態の遷移を実現している

### 4. `main` 関数の実行フロー

1. `let mut post = new Post::new();` で `Post` インスタンスを作成。初期状態は `Draft`
2. `post.add_text("...");` でテキストを追加するが、まだ下書き状態なので `post_content()` は空文字列を返す
3. `post.request_review();` で `Draft` の `request_review` メソッドが呼ばれ、状態が `Pending` になる。 `post.content` は依然として空文字列を返す
4. `post_approve();` で `PendingReview` の `approve` が呼ばれ、状態が `Published` になる
5. `assert_eq!(text, post.content());` において、これで `post_content()` が実際のコンテンツを返すようになり、アサーションが通る
