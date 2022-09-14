use std::mem;

trait Bird {
    fn fly(&self);
}

struct Duck;

struct Swan;

impl Bird for Duck {
    fn fly(&self) {
        println!("duck duck");
    }
}

impl Bird for Swan {
    fn fly(&self) {
        println!("swan swan");
    }
}

// 通过 impl Trait 实现静态分派
fn static_dispatch_fly1(bird: &impl Bird) {
    bird.fly();
}

/// 通过泛型实现静态分派
fn static_dispatch_fly2<T: Bird>(t: &T) {
    t.fly();
}

/// 实现动态分派
fn dynamic_dispatch_fly(bird: &dyn Bird) {
    bird.fly();
}

fn create_bird(name: &str) -> Option<Box<dyn Bird>> {
    match name {
        "duck" => { Some(Box::new(Duck)) }
        "swan" => { Some(Box::new(Swan)) }
        _ => { None }
    }
}

#[test]
#[allow(unused)]
fn test_static_and_dynamic_dispatch() {
    let duck = Duck;
    let swan = Swan;
    println!("static_dispatch_fly1(bird: &impl Bird)");
    static_dispatch_fly1(&duck);
    static_dispatch_fly1(&swan);
    println!();

    println!("static_dispatch_fly2<T: Bird>(t: &T)");
    static_dispatch_fly2(&duck);
    static_dispatch_fly2(&swan);
    println!();

    println!("dynamic_dispatch_fly(bird: Box<&dyn Bird>)");
    dynamic_dispatch_fly(&duck);
    dynamic_dispatch_fly(&swan);
    println!();


    println!("create_bird(name) -> Option<Box<dyn Bird>>");
    let bird = create_bird("duck");
    match bird {
        None => {}
        Some(b) => {
            b.fly();
        }
    }
}

fn print_trait_object(p: &dyn Bird) {
    let (data, vtable): (usize, *const usize) = unsafe { mem::transmute(p) };
    println!("Trait Object [data: {} vtable: {:?}]", data, vtable);
    unsafe {
        println!("data in vtable [{}, {}, {}, {}]",
                 *vtable, *vtable.offset(1), *vtable.offset(2), *vtable.offset(3));
    }
}

#[test]
#[allow(unused)]
fn test_trait_object() {
    let duck = Duck;
    // 直接对对象取指针是一个普通指针，占8个字节
    let p_duck = &duck;
    // 对象指针转换为 trait object 类型指针，占16个字节，8个字节为普通的指针指向对象内存地址，8个为虚函数表
    let p_bird = p_duck as &dyn Bird;
    println!("Size of p_duck {}, Size of p_bird {}",
             mem::size_of_val(&p_duck), mem::size_of_val(&p_bird));
    let duck_fly: usize = Duck::fly as usize;
    let swan_fly: usize = Swan::fly as usize;
    println!("Duck::fly {}", duck_fly);
    println!("Swan::fly {}", swan_fly);
    print_trait_object(p_bird);
    print_trait_object(p_duck);
    let swan = Swan;
    print_trait_object(&swan);
}
