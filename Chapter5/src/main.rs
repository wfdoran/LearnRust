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

fn main() {
    chap5_1();
}
