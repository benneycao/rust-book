#[test]
fn test_simple_closure() {
    let c = |x| -> i32 { x + 1 };
    let c1: i32 = c(1);
    println!("c1 = {}" ,c1);
}