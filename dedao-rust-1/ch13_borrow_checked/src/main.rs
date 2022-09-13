fn main() {
    println!("Hello, world!");
}

// 内存不安全示例：迭代器失效
#[test]
#[allow(unused)]
fn test_iter() {
    let mut arr = vec!["abc", "def", "ghi"];
    // &arr immutable reference
    // arr.clear() need a mutable reference
    /*    for item in &arr {
            arr.clear();
        }
    */
    eprintln!("arr = {:?}", arr);
}
// 内存不安全示例：悬空指针

#[test]
#[allow(unused)]
fn test_vec() {
    let mut arr = vec![1, 2, 3, 4, 5];
    // let p = &arr[0]; // immutable borrow occurs here
    for i in 1..100 {
        arr.push(i); // mutable borrow occurs here
    }
    // eprintln!("p = {:?}", p); // immutable borrow later used here
    // 存在一个 alias 的情况下即一个共享引用不能修改原来的值
}

#[test]
#[allow(unused)]
fn test_borrow() {
    let mut i = 100;
    // let p = &i; // immutable borrow occurs here
    i = 1000; // change the origin value; assignment to borrowed i occurs here
    eprintln!("i = {:?}", i);
}