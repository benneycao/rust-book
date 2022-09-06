use std::fmt::{Display, Formatter};

/// 使用 `?` 操作符简单的处理 `Result`
/// ```rust
/// write!(f, "{}", value)?;
/// ```
/// 定义一个包含单个 `Vec` 结构体的 `List`.然后为这个结构体实现 `Display`
struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 获取元组的值
        let v = &self.0;

        write!(f, "[")?;

        // 迭代 vec，使用 `count` 记录迭代次数
        for (count, v) in v.iter().enumerate() {
            // 对每个元素除第一个元素加上 `,`
            // 使用 `?` 来返回错误
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

#[test]
fn test_list_implement_display() {
    let list = List(vec![1, 2, 3, 4, 5, 6]);
    println!("list = {list}");
}
