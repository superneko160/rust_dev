#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub email: String,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
}

impl User {
    // ユーザ情報の表示
    pub fn display_profile(&self) -> String {
        format!("User: {} ({}) - Email: {}",
            self.name, self.age, self.email)
    }

    // アドレスが設定されているかチェック
    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }
}

#[derive(Debug)]
pub struct UserBuilder {
    pub name: String,
    pub age: u32,
    pub email: String,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
}

impl UserBuilder {
    pub fn new(name: impl Into<String>, age: u32, email: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            age,
            email: email.into(),
            gender: None,
            address: None,
            phone_number: None,
        }
    }

    // 任意フィールド（プロパティ）のsetter
    pub fn gender(mut self, gender: impl Into<String>) -> Self {
        self.gender = Some(gender.into());
        self
    }

    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn phone_number(mut self, phone_number: impl Into<String>) -> Self {
        self.phone_number = Some(phone_number.into());
        self
    }

    // Userオブジェクトを構築する
    pub fn build(self) -> User {
        User {
            name: self.name,
            age: self.age,
            email: self.email,
            gender: self.gender,
            address: self.address,
            phone_number: self.phone_number,
        }
    }
}

fn main() {
    // 1. 基本的な設定
    let user1 = UserBuilder::new("Bob", 32, "bob@example.com")
        .address("California")
        .gender("male")
        .phone_number("090-1234-5678")
        .build();

    println!("{}", user1.display_profile());
    println!("Has address: {}\n", user1.has_address());

    // 2. 部分的な設定
    let user2 = UserBuilder::new("Alice", 31, "alice@example.com")
        .address("Massachusetts")
        .build();

    println!("{:?}\n", user2);

    // 3. メソッドチェーンの柔軟性
    let builder = UserBuilder::new("Eve", 24, "eve@example.com");

    // 条件によって異なる設定
    let user3 = if true {  // 何らかの条件
        builder.gender("female").address("New Jersey")
    } else {
        builder.phone_number("080-0000-0000")
    }.build();

    println!("{:?}\n", user3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_builder() {
        let user = UserBuilder::new("Test User", 24, "test@example.com")
            .address("Test Address")
            .build();

        assert_eq!(user.name, "Test User");
        assert_eq!(user.age, 24);
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.address, Some("Test Address".to_string()));
        assert!(user.has_address());
    }
}
