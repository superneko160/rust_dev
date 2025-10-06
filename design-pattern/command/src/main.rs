mod command;
mod api_service;
mod commands;
mod api_client;

use api_service::ApiService;
use commands::{GetUserDataCommand, LogoutUserCommand};
use api_client::ApiClient;

fn main() {
    // 1. Receiver（API処理の実体）を作成
    let api_service = ApiService::new("https://api.example.com/v1");
    let user_id_to_operate = 12345;

    // 2. Invoker（APIクライアント）を作成
    let mut api_client = ApiClient::new();

    // 登録されていないコマンドの実行を試みる
    api_client.send_request("non_existent");
    println!();

    // 3. ConcreteCommand（APIリクエスト）を作成し、APIクライアントに登録
    let get_user_cmd = GetUserDataCommand::new(api_service.clone(), user_id_to_operate);
    api_client.register_command("getUserData", Box::new(get_user_cmd));
    let logout_cmd = LogoutUserCommand::new(api_service.clone(), user_id_to_operate);
    api_client.register_command("logoutUser", Box::new(logout_cmd));
    println!();

    // 4. Invokerを通じてコマンドを実行
    api_client.send_request("getUserData");
    println!();
    api_client.send_request("logoutUser");
    println!();
}
