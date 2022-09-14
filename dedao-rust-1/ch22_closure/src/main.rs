mod closure_type;
mod closure_as_parameter;
mod closure_as_return_type;
mod closure_lifetime;
mod static_dynamic_dispatch;

fn main() {
    println!("hello");
}

#[test]
#[allow(unused)]
fn test_multiple_anonymous_struct() {
    // 每个 closure 都是一个 anonymous struct
    let mut closuer = |x: i32| -> i32 { x + 2 };
    // closuer = |x: i32| -> i32{ x - 2 };
    println!("{}", closuer(1));
}
