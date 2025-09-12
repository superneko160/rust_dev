use std::collections::HashMap;
use chrono::Local;
use crate::models::UserModel;

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
