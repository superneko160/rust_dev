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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn test_get_user_by_id() {
        let adapter = UserServiceAdapter::new();
        let user_id = 123;
        let result = adapter.get_user_by_id(user_id);

        // 必要なキーが存在するか確認
        assert!(result.contains_key("user_id"));
        assert!(result.contains_key("user_name"));
        assert!(result.contains_key("user_age"));
        assert!(result.contains_key("created_at"));

        assert_eq!(result.get("user_id").unwrap(), &user_id.to_string());
        assert_eq!(result.get("user_name").unwrap(), "Alice");  // UserModel::findの固定値
        assert_eq!(result.get("user_age").unwrap(), "12");  // UserModel::findの固定値

        let created_at = result.get("created_at").unwrap();
        assert!(DateTime::parse_from_rfc3339(created_at).is_ok());
    }

    #[test]
    fn test_create_user() {
        let adapter = UserServiceAdapter::new();
        let name = "Bob".to_string();
        let age = 25;
        let result = adapter.create_user(name.clone(), age);
        
        // 必要なキーが存在するか確認
        assert!(result.contains_key("user_id"));
        assert!(result.contains_key("user_name"));
        assert!(result.contains_key("user_age"));
        assert!(result.contains_key("status"));
        assert!(result.contains_key("created_at"));
        
        assert_eq!(result.get("user_id").unwrap(), "99"); // UserModel::createの固定値
        assert_eq!(result.get("user_name").unwrap(), &name);
        assert_eq!(result.get("user_age").unwrap(), &age.to_string());
        assert_eq!(result.get("status").unwrap(), "CREATED");
        
        let created_at = result.get("created_at").unwrap();
        assert!(DateTime::parse_from_rfc3339(created_at).is_ok());
    }
}
