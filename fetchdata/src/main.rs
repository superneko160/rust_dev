use serde::{Deserialize, Serialize};
use reqwest::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    id: i32,
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
    username: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // TODOデータを取得して表示
    let todos = fetch_todos().await?;
    display_todos(&todos);

    println!("=============");

    // 特定の投稿を取得して表示
    let post = fetch_post(1).await?;
    display_post(&post);

    println!("=============");

    // ユーザー情報を取得して表示
    let users = fetch_users().await?;
    display_users(&users);

    Ok(())
}

// TODOデータを取得する関数
async fn fetch_todos() -> Result<Vec<Todo>, Error> {
    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos")
        .send()
        .await?
        .json()
        .await?;
    
    Ok(todos)
}

// TODOデータを表示する関数
fn display_todos(todos: &[Todo]) {
    println!("First 3 todos:");
    for todo in todos.iter().take(3) {
        println!("{:#?}", todo);
    }
}

// 特定の投稿を取得する関数
async fn fetch_post(id: i32) -> Result<Post, Error> {
    let post: Post = reqwest::Client::new()
        .get(&format!("https://jsonplaceholder.typicode.com/posts/{}", id))
        .send()
        .await?
        .json()
        .await?;
    
    Ok(post)
}

// 投稿を表示する関数
fn display_post(post: &Post) {
    println!("Post #{}:", post.id);
    println!("{:#?}", post);
}

// ユーザー情報を取得する関数
async fn fetch_users() -> Result<Vec<User>, Error> {
    let users: Vec<User> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/users")
        .send()
        .await?
        .json()
        .await?;
    
    Ok(users)
}

// ユーザー情報を表示する関数
fn display_users(users: &[User]) {
    println!("First 2 users:");
    for user in users.iter().take(2) {
        println!("{:#?}", user);
    }
}
