/**
- `format!`: write formatted text to `String`
- `print!`: same as `format!` but the text is printed to the console.
- `println!`: same as `print!` but a newline is appended.
- `eprint!`: same as `format!` but the text is printed to the standard error
- `eprintln!`: same as `eprint!` but a newline is appended
 */
#[test]
fn test_formatter() {
    // 默认情况
    println!("days {}", 31);
    // 可以使用位置参数
    println!("{0}, this is {1}. {1}, this is {0},", "Alice", "Bob");
    // 可以使用命名参数
    println!(
        "{subject} {verb} {object}",
        subject = "the quick brown fox",
        object = "the lazy dog",
        verb = "jumps over"
    );
    // 还可以指定输出格式
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
}
