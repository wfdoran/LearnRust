fn main() {
    sub1();
    sub2();
    sub3();
}


fn sub1() {
    let my_number: u8 = 100;
    println!("{}", my_number as char);
}

fn sub2() {
    println!("size of a char: {}", std::mem::size_of::<char>());
    println!("size of \"a\": {}", "a".len());
}

fn sub3() {
    let a = 10_u8;
    let b = 10_u16;
    let c = 10_u32;
    let d = 10_u64;
    let e = 10_u128;

    println!("{} {} {} {} {}", a, b, c, d, e);

    let f = 3.0;
    let g = 3.0_f32;

    println!("{} {}", 1.0/f, 1.0/g);

    println!("{}", 1.0 / give_number() as f64);

    println!("{}", multiply(8, 5));
}

fn give_number() -> i32 {
    7
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}