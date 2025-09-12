// Adaptee - 既存のユーザーモデル
#[derive(Debug, Clone)]
pub struct UserModel {
    pub id: u32,
    pub name: String,
    pub age: Option<u32>,
}

impl UserModel {
    // データの取得
    pub fn find(id: u32) -> UserModel {
        // 仮データ
        UserModel {
            id,
            name: "Alice".to_string(),
            age: Some(12),
        }
    }

    // データの作成
    pub fn create(name: String, age: u32) -> UserModel {
        // 仮データ
        UserModel {
            id: 99,
            name,
            age: Some(age),
        }
    }
}
