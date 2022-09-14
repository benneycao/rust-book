use std::fmt::Display;

fn multiply(m: i32) -> impl Fn(i32) -> i32 {
    move |x| x * m
}

fn display<T: Display>(t: T) -> impl Display {
    t
}

#[test]
#[allow(unused)]
fn test_multiply() {
    let f = multiply(5);
    let display1 = display(1);
    println!("{}", display1);
    println!("{}", f(2));
}
