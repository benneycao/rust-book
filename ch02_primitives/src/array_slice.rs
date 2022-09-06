/// array `[T; length]` 是一组拥有相同类型 `T` 的对象的集合，在内存中是连续存储的。
/// slice `&[T] &mut [T]` 类型和数组类似，但其大小在编译时不确定。切片是一个双字节对象，第一个是指向数据的指针，第二个是切片的长度。
#[test]
fn test_array_slice() {
    // 定义一个数组
    let arr = [1, 2, 3, 4, 5];
    eprintln!("arr = {:?}", arr);
    
    // 引用数组，创建一个数组的全切片
    let slice = &arr;
    eprintln!("slice = {:?}", slice);
    
    // 引用数组部份，从索引 1 开始至结束
    let slice = &arr[1..];
    eprintln!("slice = {:?}", slice);
    
    // 从开头引用至索引为3的元素
    let slice = &arr[0..4];
    eprintln!("slice = {:?}", slice);
    
    // 引用数组的中间部份如 1..3 表示从索引1开始引用长度为2
    let slice = &arr[1..3];
    eprintln!("slice = {:?}", slice);
}
