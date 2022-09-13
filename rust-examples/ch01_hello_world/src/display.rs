use std::fmt::{Display, Formatter};

/// `Debug trait` 通常看起来不太简洁，因此需要自定义的输出外观，可以实现 `Display trait`. `Display` 不能通过
/// `#[derive]`宏来派生，必须手动实现
struct MyI32(i32);

impl Display for MyI32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_display() {
    let my_i32 = MyI32(100);
    println!("my_i32 is {my_i32}");
}