#![allow(dead_code)]
use rand::Rng;

/// 在随机数生成器 `rand::Rng`，通过`rand::thread_rng`生成随机数
#[test]
fn test_rng() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

#[test]
fn test_rng_range() {
    // 使用 `Rng::gen_range` 选取范围
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}
