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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_model_fields() {
        let user = UserModel {
            id: 123,
            name: "Test User".to_string(),
            age: Some(31),
        };

        assert_eq!(user.id, 123);
        assert_eq!(user.name, "Test User");
        assert_eq!(user.age, Some(31));
    }

    #[test]
    fn test_user_model_with_none_age() {
        let user = UserModel {
            id: 123,
            name: "Ageless User".to_string(),
            age: None,
        };

        assert_eq!(user.id, 123);
        assert_eq!(user.name, "Ageless User");
        assert_eq!(user.age, None);
    }

    #[test]
    fn test_find_user() {
        let user_id = 1;
        let user = UserModel::find(user_id);

        assert_eq!(user.id, user_id);
        assert_eq!(user.name, "Alice");  // 固定値
        assert_eq!(user.age, Some(12));  // 固定値
    }

    #[test]
    fn test_find_user_with_different_id() {
        let user_id = 999;
        let user = UserModel::find(user_id);

        // IDは渡された値が反映され、それ以外は固定で返ってくるのを確認
        assert_eq!(user.id, user_id);
        assert_eq!(user.name, "Alice");  // 固定値
        assert_eq!(user.age, Some(12));  // 固定値
    }

    #[test]
    fn test_create_user() {
        let name = "Bob".to_string();
        let age = 24;
        let user = UserModel::create(name.clone(), age);

        // IDは固定された値が返ってきて、それ以外は渡した値で返ってくるのを確認
        assert_eq!(user.id, 99);  // 固定値
        assert_eq!(user.name, name);
        assert_eq!(user.age, Some(age));
    }

    #[test]
    fn test_create_user_with_different_values() {
        let name = "Charlie".to_string();
        let age = 33;
        let user = UserModel::create(name.clone(), age);

        // IDは固定された値が返ってきて、それ以外は渡した値で返ってくるのを確認
        assert_eq!(user.id, 99);  // 固定値
        assert_eq!(user.name, name);
        assert_eq!(user.age, Some(age));
    }
}
