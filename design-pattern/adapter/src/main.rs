use adapter::controllers::UserController;
use adapter::services::UserServiceAdapter;

fn main() {
    let adapter = UserServiceAdapter::new();
    let controller = UserController::new(adapter);

    // ユーザ情報の表示
    controller.show_user(1);

    // ユーザ情報の登録
    controller.register_user("Bob".to_string(), 24);
}
