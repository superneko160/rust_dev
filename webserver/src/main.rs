use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::fs::File;
use std::thread;
use std::time::Duration;

/**
 * ブラウザでindex.htmlを開く(http://localhost:7878)
 * ポートフォワードの設定：.devcontainer/devcontainer.json
 * http://localhost:7878/sleepが読み込まれている間も、http://localhost:7878/が開ける（更新できる）
 * TODO: 無制限にスレッドを生成してしまうため修正する必要有
 */
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // 新しいスレッドを生成し、クロージャ内のコードを新しいスレッドで実行 
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";  // スレッドが生成されているか確認する用

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "/workspaces/rust_dev/webserver/html/hello.html")
    } else if buffer.starts_with(sleep) {  // スレッドが生成されているか確認する用
        thread::sleep(Duration::from_secs(5));  // 5秒待機
        ("HTTP/1.1 200 OK\r\n\r\n", "/workspaces/rust_dev/webserver/html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "/workspaces/rust_dev/webserver/html/404.html")
    };
    let mut file = File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}