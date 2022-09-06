/// 可以使用 `#[derive(Debug)]` 来派生 `Debug trait`，如果结构体的所有的字段/成员都实现了 `Debug trait`
#[test]
fn test_debug() {
    #[derive(Debug)]
    struct Hello {
        word: String,
    }
    let hello = Hello { word: "Hello World!".to_string() };

    // 打印实现 Debug trait 的 String type
    println!("word is {:?}", hello.word);

    // 普通打印
    eprintln!("hello = {:?}", hello);

    // 美化打印方式
    eprintln!("hello = {:#?}", hello);
}