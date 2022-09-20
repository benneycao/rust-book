use std::thread;
use std::time::Duration;
use futures::executor::block_on;
async fn do_somethings() {
    println!("go go go ");
    println!("hay hay hay ");
}

async fn hello() {
    println!("hello ");
    world().await;
    println!("hello1 ");
    println!("hello2 ");
    println!("hello3 ");
}

async fn world() {
    thread::sleep(Duration::from_secs(1));
    println!("world");
}

#[test]
#[allow(unused)]
fn test_async_demo() {
    let somethings = do_somethings();

    block_on(somethings)
    // do_somethings();
}

#[test]
#[allow(unused)]
/// 测试一个 async function invoke anther async function and use `await` keywords
fn test_async_demo1() {
    let hello1 = hello();
    block_on(hello1)

}