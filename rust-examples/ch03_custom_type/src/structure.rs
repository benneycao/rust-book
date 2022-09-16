#![allow(dead_code)]
/// 结构体的三种类型：
/// - `tuple struct`: 具名 tuple
/// - `C struct`: 经典的 C 语言结构体
/// - `unit struct`: 不带字段，在泛型中很有用
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// unit struct
struct Unit;

// tuple struct
#[derive(Debug)]
struct Point(i32, i32);

// 结构体可以作为另一个结构体的字段
#[derive(Debug)]
struct Rectangle {
    // 通过左上角和右下角坐标来确定一个 rectangle
    top_left: Point,
    bottom_right: Point,
}

#[test]
fn test_struct() {
    let name = String::from("cjx");
    let age = 26;
    // 创建对象的简写形式
    // let person = Person {name: name, age: age};
    let person = Person { name, age };
    eprintln!("person = {:?}", person);

    let left_top = Point(0, 10);
    let bottom_right = Point(10, 0);
    let rectangle = Rectangle {
        top_left: left_top,
        bottom_right,
    };
    eprintln!("rectangle = {:?}", rectangle);
}
