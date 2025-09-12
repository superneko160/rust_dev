use crate::services::UserService;

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
