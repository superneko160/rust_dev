use futures::executor;

async fn async_add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

async fn something_great_async_func() -> i32 {
    let ans1 = async_add(2, 3).await;  // 5
    let ans2 = async_add(3, 4).await;  // 7
    let ans3 = async_add(4, 5).await;  // 9
    let result = ans1 + ans2 + ans3;  // 21
    result
}

fn main() {
    // excutor::block_on() 渡したFutureが完了になるまでブロックして待つメソッド
    println!("{}", executor::block_on(something_great_async_func()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_add() {
        let result = async_add(2, 3).await;
        assert_eq!(result, 5);
    }

    #[tokio::test]
    async fn test_something_great_async_func() {
        let result = something_great_async_func().await;
        assert_eq!(result, 21);
    }
}
