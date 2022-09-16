use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
#[allow(unused)]
fn test_thread() {
    let numbers = (0..100u32).collect::<Vec<u32>>();
    // 引用计数，可以在多线程环境下使用
    let shared_numbers = Arc::new(numbers);
    for _ in 0..10 {
        // 复制引用计数指针，所有的的 Arc 都指向同一个 vec
        let child_numbers = shared_numbers.clone();
        // move 修饰 closure，上面的 arc 指针被 Move 到新的线程中
        thread::spawn(move || {
            let local_numbers = &child_numbers[..];
            eprintln!("local_numbers = {:?}", local_numbers);
        });
    }
    thread::sleep(Duration::from_secs(1));
}
