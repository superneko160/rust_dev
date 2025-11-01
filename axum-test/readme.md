# axum

Rust's framework

## Access

- http://localhost:7878/
- http://localhost:7878/list?page=2&per_page=33
- http://localhost:7878/users/95be61c6-ffdc-4283-8d3b-a5048a53cfbb
- http://localhost:7878/contact

### Post Data

```sh
curl -X POST -H "Content-Type: application/json" -d '{"name":"Alice"}' http://localhost:7878/users
```

##  Extractor

- `axum::extract::Path` : パスパラメータ（URL 内のパスの部分的なセグメント）を抽出
- `axum::extract::Query` : URL の末尾のクエリパラメータを抽出
- `axum::extract::Json` : POST メソッドのペイロードとして送られてきた JSON データを抽出
- `axum::extract::Request` : リクエスト全体を抽出
