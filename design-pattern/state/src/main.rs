// 以下は、ブログ投稿のワークフロー
// 1. 空の下書きをベースとして開始される
// 2. 下書きが完成すると、投稿レビューが要求される
// 3. 投稿が承認されると公開される
// 4. 公開された投稿のみが印刷用コンテンツを返すため、未承認の投稿が誤って公開されることはない

// --- 状態が持つ振る舞い ---
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post:&'a Post) -> &'a str {
        ""
    }
}
// --- 状態が持つ振る舞い ---

// --- 具体的な状態 ---
// 下書き状態
struct Draft;

impl State for Draft {
    fn request_review(self: Box<Draft>) -> Box<dyn State> {
        Box::new(PendingReview {})  // PendingReview状態に遷移
    }

    fn approve(self: Box<Draft>) -> Box<dyn State> {
        self
    }
}

// レビュー待ち状態
struct PendingReview;

impl State for PendingReview {
    fn request_review(self: Box<PendingReview>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<PendingReview>) -> Box<dyn State> {
        Box::new(Published {})  // Published状態に遷移
    }
}

// 公開済み状態
struct Published;

impl State for Published {
    fn request_review(self: Box<Published>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Published>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
// --- 具体的な状態 ---

// --- 状態を管理するコンテキスト ---
// Postは投稿のコンテキストであり、現在の状態オブジェクトを保持
// Post自身は多くの条件分岐を持つことなく、状態の変化と振る舞いを状態オブジェクトに任せている
struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
// --- 状態を管理するコンテキスト ---

fn main() {
    let mut post = Post::new();

    let text = "State is behavioral design pattern.";
    post.add_text(text);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(text, post.content());
    println!("post content {}", post.content());
}
