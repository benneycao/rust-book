fn is_strong_1(password: String) -> bool {
    password.len() > 6
}

fn is_strong_2(pw: &str) -> bool {
    pw.len() > 6
}

fn is_strong_3(pw: impl Into<String>) -> bool {
    pw.into().len() > 6
}

#[test]
#[allow(unused)]
fn test_is_strong() {
    let pw = "hello";
    let is = is_strong_1(pw.into());
    eprintln!("is = {:?}", is);
    let is = is_strong_2(pw);
    eprintln!("is = {:?}", is);
    let is = is_strong_3(pw);
    eprintln!("is = {:?}", is);
}
