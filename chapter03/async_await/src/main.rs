use futures::executor;
use std::future::Future;

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn something_great_async_function() -> i32 {
    let ans1 = async_add(2, 3).await;
    let ans2 = async_add(3, 4).await;
    let ans3 = async_add(4, 5).await;
    let result = ans1 + ans2 + ans3;
    println!("{}", result);
    result
}

fn move_to_async_block() -> impl Future<Output = ()> {
    let outside_variable = "this is outside".to_string();
    async move {
        println!("{}", outside_variable);
    }
}

async fn some_great_function(arg: &i32) -> i32 {
    *arg
}

fn main() {
    executor::block_on(something_great_async_function());
    let _ = move_to_async_block();
    // println!("some_gread_function: {}", something_great_async_function());
    let result = executor::block_on(some_great_function(&1));
    println!("some_gread_function: {}", result);
}
