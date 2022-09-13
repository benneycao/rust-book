fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

#[test]
fn test_first_word() {
    let mut s = String::from("helloworld");
    let word = first_word(&s);
    s.clear();
    eprintln!("word = {:?}", word);
    // word also have value, but the s variable value is clear
}

/// this function return a string slice -> &str,
/// slice can not live alone. slice dependence other object to live
fn first_world_with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

#[test]
fn test_first_world_with_slice() {
    let mut s = String::from("hello world");
    let word = first_world_with_slice(&s);
    /// why word is the variable s immutable reference
    /// because the slice cannot live alone, it must dependence other object
    /// so word reference the variable s, so word is the s immutable reference
    /// pub fn clear(&mut self)
    /// this clear function need send mutable reference,
    /// now we have a immutable reference and a mutable reference in same scope, so cannot allowed to compile success
    // s.clear(); error
    eprintln!("word = {:?}", word);
}
