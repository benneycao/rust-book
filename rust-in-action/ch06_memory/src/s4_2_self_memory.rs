use std::mem::forget;

static GLOBAL: i32 = 1000;

fn noop() -> *const i32 {
    let noop_local = 12345;
    &noop_local as *const i32
}

#[test]
#[allow(unused)]
fn test_noop() {
    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new("b");
    let boxed_int = Box::new(789);
    let fn_int = noop();
    println!("GLOBAL:           {:p}", &GLOBAL as *const i32);
    println!("GLOBAL value:     {}", unsafe { *(&GLOBAL as *const i32) });
    println!("local_str:        {:p}", local_str as *const str);
    println!("local_str value:  {}", unsafe { &*(local_str as *const str) });
    println!("local_int:        {:p}", &local_int as *const i32);
    println!("local_int value:  {}", unsafe { *(&local_int as *const i32) });
    println!("boxed_str:        {:p}", Box::into_raw(boxed_str));
    println!("boxed_int:        {:p}", Box::into_raw(boxed_int));
    println!("fn_int:           {:p}", fn_int);
    /// 期望值 -> 1234 实际解引用值：-> 1 原因为原内存上的值被析构了,值被释放了，现在内存上的值是随机的
    println!("fn_int value:     {}", unsafe { *fn_int });
}