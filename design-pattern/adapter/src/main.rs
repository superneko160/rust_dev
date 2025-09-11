use std::collections::HashMap;
use chrono::Local;

// Adaptee
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

// Target（クライアントが期待するインターフェース）
pub trait UserService {
    fn get_user_by_id(&self, id: u32) -> HashMap<String, String>;
    fn create_user(&self, name: String, age: u32) -> HashMap<String, String>;
    fn get_user_display_name(&self, id: u32) -> String;
}

// Adapter（UserModelを異なるインターフェースで使えるようにする）
pub struct UserServiceAdapter;

impl UserServiceAdapter {
    pub fn new() -> Self {
        UserServiceAdapter
    }
}

impl UserService for UserServiceAdapter {
    fn get_user_by_id(&self, id: u32) -> HashMap<String, String> {
        let user = UserModel::find(id);
        let mut result = HashMap::new();

        result.insert("user_id".to_string(), user.id.to_string());
        result.insert("user_name".to_string(), user.name);
        result.insert("user_age".to_string(), user.age.map_or(
                "未設定".to_string(), |age| age.to_string()));
        result.insert("created_at".to_string(), Local::now().to_rfc3339());

        result
    }

    fn create_user(&self, name: String, age: u32) -> HashMap<String, String> {
        let user = UserModel::create(name, age);
        let mut result = HashMap::new();

        result.insert("user_id".to_string(), user.id.to_string());
        result.insert("user_name".to_string(), user.name);
        result.insert("user_age".to_string(), user.age.map_or(
                "未設定".to_string(), |age| age.to_string()));
        result.insert("status".to_string(), "CREATED".to_string());
        result.insert("created_at".to_string(), Local::now().to_rfc3339());

        result
    }

    fn get_user_display_name(&self, id: u32) -> String {
        let user = UserModel::find(id);
        format!("<{}>", user.name)
    }
}

// Client（Adapterを利用する側）
pub struct UserController<T: UserService> {
    user_service: T,
}

impl<T: UserService> UserController<T> {
    pub fn new(user_service: T) -> Self {
        UserController { user_service }
    }

    pub fn show_user(&self, id: u32) {
        let user_data = self.user_service.get_user_by_id(id);
        let display_name = self.user_service.get_user_display_name(id);

        println!(
            "{}: {} ({})",
            user_data.get("user_id").unwrap_or(&"?".to_string()),
            display_name,
            user_data.get("user_age").unwrap_or(&"未設定".to_string()),
        );
        println!(
            "登録日時: {}",
            user_data.get("created_at").unwrap_or(&"?".to_string()),
        );
        println!("-----");
    }

    pub fn register_user(&self, name: String, age: u32) {
        let result = self.user_service.create_user(name, age);

        println!("[ユーザ登録完了]");
        println!(
            "{}: {} ({})",
            result.get("user_id").unwrap_or(&"?".to_string()),
            result.get("user_name").unwrap_or(&"未設定".to_string()),
            result.get("user_age").unwrap_or(&"未設定".to_string()),
        );
        println!(
            "ステータス: {}",
            result.get("status").unwrap_or(&"?".to_string()),
        );
        println!("-----");
    }
}

fn main() {
    let adapter = UserServiceAdapter::new();
    let controller = UserController::new(adapter);

    // ユーザ情報の表示
    controller.show_user(1);

    // ユーザ情報の登録
    controller.register_user("Bob".to_string(), 24);
}
