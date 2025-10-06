// API処理の結果を表す型
// 実際のAPIではResult<T, E>を使うことが多いが、処理をシンプルにするため、ここでは簡易的にboolを使用
pub type ApiResult = bool;

// Commandトレイト: 実行可能な振る舞いを定義
pub trait Command {
    // コマンドを実行し、API処理の結果を返す
    fn execute(&self) -> ApiResult;
}
