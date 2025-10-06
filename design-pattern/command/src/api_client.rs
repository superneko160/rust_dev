use std::collections::HashMap;
use crate::command::Command;

// Invoker（インヴォーカ）: コマンドを保持・実行する主体
// ここではAPIリクエストをキューイング・実行する役割を担う
pub struct ApiClient {
    commands: HashMap<String, Box<dyn Command>>,
}

impl ApiClient {
    pub fn new() -> ApiClient {
        ApiClient {
            commands: HashMap::new(),
        }
    }

    // コマンドを登録する
    pub fn register_command(&mut self, name: &str, cmd: Box<dyn Command>) {
        self.commands.insert(name.to_string(), cmd);
        println!("Client: Command '{}' registered.", name);
    }

    // コマンド名で登録された処理を実行する
    pub fn send_request(&self, name: &str) {
        println!("Client: Sending request for command '{}'...", name);
        if let Some(cmd) = self.commands.get(name) {
            let result = cmd.execute();
            if result {
                println!("Client: Command execution SUCCESS.");
            } else {
                println!("Client: Command execution FAILED.");
            }
        } else {
            println!("Client: Command '{}' not found. Doing nothing.", name);
        }
    }
}
