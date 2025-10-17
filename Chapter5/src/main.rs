use std::fmt::Debug;
use std::fmt::Display;
use std::cmp::PartialOrd;

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}


fn print_item<T: Debug>(item: T) {
    println!("Your item is {item:?}");
}

fn cmp_and_display<T, U>(stmt: T, x: U, y: U) 
where
  T: Display,
  U: Display + PartialOrd
{
    if x > y {
        println!("{stmt}: {x}");
    } else {
        println!("{stmt}: {y}");
    }
}

fn chap5_1() {
    print_item(5);
    print_item(10.4);
    print_item("hello");

    let x = Animal {
        name: "Sandy".to_string(),
        age: 15,
    };

    print_item(x);

    cmp_and_display("aaa", 5, 2);
    cmp_and_display("bbb", 2.5, 7.3);
}


fn take_fifth_item(value: &Vec<i32>) -> Option<i32> {
    if (value.len() < 5) {
        None
    } else {
        Some(value[4])
    }
}

fn print_value(x: Option<i32>) {
    match x {
        Some(a) => println!("found {a}"),
        None => println!("found None!"),
    }
}

fn chap5_2_1() {
    let a = vec![1,2];
    let b = vec![3,4,5,6,7,8];
    println!("{:?} {:?}", take_fifth_item(&a), take_fifth_item(&b));
    print_value(take_fifth_item(&a));
    print_value(take_fifth_item(&b));
    println!("{} {}", take_fifth_item(&a).is_none(), take_fifth_item(&a).is_some());

}

fn main() {
    chap5_1();
    chap5_2_1();
}
