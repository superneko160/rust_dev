use crate::command::{ApiResult, Command};
use crate::api_service::ApiService;

// ConcreteCommand（具象コマンド）: ユーザデータ取得コマンド
pub struct GetUserDataCommand {
    api_service: ApiService,
    user_id: u32,
}

impl GetUserDataCommand {
    pub fn new(api_service: ApiService, user_id: u32) -> GetUserDataCommand {
        GetUserDataCommand { api_service, user_id }
    }
}

impl Command for GetUserDataCommand {
    fn execute(&self) -> ApiResult {
        // execute内でレシーバのメソッドを呼び出す
        self.api_service.get_user_data(self.user_id)
    }
}

// ConcreteCommand（具象コマンド）: ユーザログアウトコマンド
pub struct LogoutUserCommand {
    api_service: ApiService,
    user_id: u32,
}

impl LogoutUserCommand {
    pub fn new(api_service: ApiService, user_id: u32) -> LogoutUserCommand {
        LogoutUserCommand { api_service, user_id }
    }
}

impl Command for LogoutUserCommand {
    fn execute(&self) -> ApiResult {
        // execute内でレシーバのメソッドを呼び出す
        self.api_service.logout_user(self.user_id)
    }
}
