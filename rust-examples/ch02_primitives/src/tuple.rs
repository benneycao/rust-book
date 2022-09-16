/// tuple 使用 `()` 来构造，每个元素都有自己的类型标记 `(T1, T2,..,Tn)`。函数可以通过 tuple 返回多个值。
/// tuple 还可以当函数的参数。
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 解构语法
    let (integer, boolean) = pair;
    // 返回元组
    (boolean, integer)
}

#[test]
fn test_tuple() {
    let pair = (1, true);
    eprintln!("pair = {:?}", pair);
    let reverse_pair = reverse(pair);
    println!("0.{} 1.{}", reverse_pair.0, reverse_pair.1);
    eprintln!("reverse_pair = {:?}", reverse_pair);
}
