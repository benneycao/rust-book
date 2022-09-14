fn call_with_closure<F>(some_closure: F, i: i32) -> i32
    where F: Fn(i32) -> i32 {
    some_closure(i)
}

#[test]
#[allow(unused)]
fn test_closure_as_parameters() {
    let answer = call_with_closure(|x| x + 1, 1);
    println!("answer = {}", answer);
}

fn static_dispatch<F>(closure: &F) where F: Fn(i32) -> i32 {
    println!("static dispatch {}", closure(42) as i32);
}

fn dynamic_dispatch(closure: &dyn Fn(i32) -> i32) {
    println!("dynamic dispatch {}", closure(42) as i32)
}

#[test]
#[allow(unused)]
fn test_send_closure() {
    let c1 = |x| x * 2;
    let c2 = |x| x * 3;
    fn function_ptr(x: i32) -> i32 {
        x * 4
    }

    static_dispatch(&c1);
    static_dispatch(&c2);
    static_dispatch(&function_ptr);

    dynamic_dispatch(&c1);
    dynamic_dispatch(&c2);
    dynamic_dispatch(&function_ptr);
}

