use std::fmt::{Debug, Display};
use std::io::Take;
use std::ops::Add;

#[test]
#[allow(unused)]
fn old_fun() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

/// now we can send any i32 type slice to this function then return the largest element in this slice
/// example i32 array or vec<i32> and other type
#[allow(unused)]
fn largest_1(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// return the largest element in i32 type slice
#[allow(unused)]
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// return the largest element in char type slice
#[allow(unused)]
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// now we use generics type to create a function can send i32 or char type to params
fn largest<T>(list: &[T]) -> T
where
    T: Copy + PartialOrd,
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[test]
fn test_largest() {
    let nums = vec![1, 2, 3, 4, 6, 7, 10, 1, 1, 1, 199];
    let largest = largest(&nums);
    eprintln!("largest = {:?}", largest);
}

/// generics in struct definitions
/// in this Point struct generics means x and y had a same type
struct Point<T> {
    x: T,
    y: T,
}

/// create different function when have different trait bound
/// create a generics function have trait bound
impl<T> Point<T>
where
    T: Display,
{
    fn show_point(&self) {
        println!("x: {}, y: {} ", self.x, self.y);
    }
}

impl<T> Point<T>
where
    T: Copy + Add<Output = T>,
{
    fn get_z(&self) -> T {
        self.x + self.y
    }
}

#[test]
fn test_point_struct() {
    let integer = Point { x: 10, y: 100 };
    let float = Point { x: 3.11, y: 3.14 };
    integer.show_point();
    float.show_point();
    println!("z = {}", integer.get_z());
    println!("z = {}", float.get_z());
}
