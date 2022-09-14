/// 1. 不希望一个 trait 通过 trait object 方式使用，可以给这个 trait 加上 Sized 约束
/// 2. 如果阻止一个函数在虚函数表中出现，可以为该函数加上 Self: Sized 约束。
/// object safe
/// 1. 如果一个trait 方法的返回值类型为 Self 也没有 trait object. 不满足 object safe
/// 2. 如果一个函数的第一个参数不是 self &self &mut self 也不满足 Object safe 同样添加 Self: Sized 移除该方法
/// 3. 如果方法有泛型，也是禁止的 不满足 object safe
trait Foo where Self: Sized {
    fn foo(&self);
}

trait Foo2 {
    fn foo1(&self);
    fn foo2(&self) where Self: Sized;
}

impl Foo for i32 {
    fn foo(&self) {
        println!("{}", self);
    }
}

impl Foo2 for i32 {
    fn foo1(&self) {
        println!("foo1 {}", self);
    }

    fn foo2(&self) where Self: Sized {
        println!("foo2 {}", self);
    }
}

#[test]
#[allow(unused)]
fn test_foo() {
    let x = 1;
    x.foo();
    // let p = &x as &dyn Foo; error
    // p.foo();
    let y = 10;
    y.foo1();
    y.foo2();
    let y = &y as &dyn Foo2;
    y.foo1();
    // y.foo2(); error
}

mod safe2 {
    use std::fmt::Display;

    /// 如果一个trait 方法的返回值类型为 Self 也没有 trait object.
    /// 如果一个函数的第一个参数不是 self &self &mut self 也不满足 Object safe 同样添加 Self: Sized 移除该方法
    /// 如果方法有泛型，也是禁止的
    trait Double {
        /// 为new 方法的 Self 添加 Sized 约束，表示该方法不会出现在 虚函数表中
        fn new() -> Self where Self: Sized;
        fn double(&mut self);
        fn show() where Self: Sized;
        fn generics<T: Display>(&self, t: T) where Self: Sized;
    }

    impl Double for i32 {
        fn new() -> Self {
            0
        }


        fn double(&mut self) {
            *self *= 2;
        }

        fn show() where Self: Sized {
            println!("show");
        }

        fn generics<T: Display>(&self, t: T) {
            println!("{}", t);
            println!("{}", self);
        }
    }

    #[test]
    #[allow(unused)]
    fn test_double() {
        let mut i = 1;
        let p = &mut i as &mut dyn Double;
        p.double();
    }
}







































