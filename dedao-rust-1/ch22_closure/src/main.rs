fn main() {
    println!("hello");
}


#[test]
#[allow(unused)]
fn test_fn_once() {
    let v: Vec<i32> = vec![1, 2, 3];
    let c = || drop(v);
    c();
    // c();
    println!("following code is upper code unfolding");
    struct ClosureEnvironment {
        _v: Vec<i32>,
    }
    impl ClosureEnvironment {
        fn call_once(self) {
            drop(self._v)
        }
    }
    let v = vec![1, 2, 3];
    let c = ClosureEnvironment { _v: v };
    c.call_once();
    // c.call_once();// error
}

#[test]
#[allow(unused)]
fn test_fn() {
    let v = vec![1, 2, 3];
    let c = |a: i32| for i in &v { println!("{}", *i + a); };
    c(1);
    println!("following code is upper code unfolding");
    struct ClosureEnvironment<'a> {
        _v: &'a Vec<i32>,
    }
    impl ClosureEnvironment<'_> {
        fn call(&self, a: i32) {
            for i in self._v {
                println!("{}", *i + a);
            }
        }
    }
    let v = vec![1, 2, 3];
    let c = ClosureEnvironment { _v: &v };
    c.call(1);
}

#[test]
#[allow(unused)]
fn test_fn_mut() {
    let mut v = vec![1, 2, 3];
    let mut c = |i| v.push(i);
    c(4);
    c(5);
    eprintln!("v = {:?}", v);
    println!("following code is upper code unfolding");
    struct ClosureEnvironment<'a> {
        _v: &'a mut Vec<i32>,
    }
    impl ClosureEnvironment<'_> {
        fn call_mut(&mut self, i: i32) {
            self._v.push(i);
        }
    }
    let mut v = vec![1, 2, 3];
    let mut c = ClosureEnvironment { _v: &mut v };
    c.call_mut(4);
    c.call_mut(5);
    eprintln!("v = {:?}", v);
}

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

#[test]
#[allow(unused)]
fn test_multiple_anonymous_struct() {
    // 每个 closure 都是一个 anonymous struct
    let mut closuer = |x: i32| -> i32 { x + 2 };
    // closuer = |x: i32| -> i32{ x - 2 };
    println!("{}", closuer(1));
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





















