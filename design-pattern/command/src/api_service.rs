use crate::command::ApiResult;

// Receiver（レシーバ）: コマンドが実行を委譲する対象
// 実際のAPI呼び出しのロジックを持つ役割
#[derive(Clone)]
pub struct ApiService {
    base_url: String,
}

impl ApiService {
    pub fn new(base_url: &str) -> ApiService {
        ApiService {
            base_url: base_url.to_string(),
        }
    }

    // ユーザ情報の取得
    pub fn get_user_data(&self, user_id: u32) -> ApiResult {
        println!("API: GET {}/users/{}", self.base_url, user_id);
        // 実際にはここで外部APIへのHTTPリクエストが発生する
        println!("-> Successfully retrieved user data for ID: {}", user_id);
        true // 成功として簡易的にtrueを返す
    }

    // ユーザのログアウト処理
    pub fn logout_user(&self, user_id: u32) -> ApiResult {
        println!("API: POST {}/logout?user_id={}", self.base_url, user_id);
        // 実際にはここで外部APIへのHTTPリクエストが発生する
        println!("-> Successfully logged out user ID: {}", user_id);
        true // 成功として簡易的にtrueを返す
    }
}
