#![allow(dead_code)]

use crate::enumeration::List::{Cons, Nil};

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    // 创建一个空的 List
    fn new() -> Self {
        Nil
    }

    // 处理一个 List 在头部插入新元素，并返回该 List
    fn prepend(self, elem: u32) -> Self {
        Cons(elem, Box::new(self))
    }
    // 返回 List 的长度
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    // 返回列表的字符串表示（该字符串是堆分配的）
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，
                // 而不是打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

#[test]
fn test_list() {
    // 创建一个空链表
    let mut list = List::new();

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}