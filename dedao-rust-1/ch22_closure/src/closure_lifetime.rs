fn calc_by<'a, F>(var: &'a i32, f: F) -> i32
    where F: Fn(&'a i32) -> i32
{
    let local = *var;
    f(&local)
}

#[test]
#[allow(unused)]
fn test_calc_by() {
    let local = 10;
    let result = calc_by(&local, |i| i * 2);
    eprintln!("result = {:?}", result);
}