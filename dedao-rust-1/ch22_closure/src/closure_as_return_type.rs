fn closure_with_return_1() -> Box<dyn Fn(i32) -> i32> {
    let c = |x: i32| -> i32 { x * 2 };
    Box::new(c)
}

fn closure_with_return_2() -> impl Fn(i32) -> i32 {
    let c = |x: i32| -> i32 { x * 2 };
    c
}


#[test]
#[allow(unused)]
fn test_closure_as_return_type() {
    println!("box dynamic");
    let c = closure_with_return_1();
    let r = c(2);
    let r1 = c(3);
    println!("{}", r as i32);
    println!("{}", r1 as i32);
    println!();
    println!("impl Fn trait");
    let c = closure_with_return_2();
    let r = c(2);
    let r1 = c(3);
    println!("{}", r);
    println!("{}", r1);
}