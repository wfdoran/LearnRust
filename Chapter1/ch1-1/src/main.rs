fn main() {
    sub1();
    sub2();
    sub3();
    sub4();
    sub5();
    sub6();
    sub7();
    sub8();
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

    println!("{}", multiply(8,5));
}

fn give_number() -> i32 {
    7
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn sub4() {
    let color1 = "red";
    let color2 = "blue";
    let color3 = "green";
    println!("{color1} {color2} {color3}");
}

fn sub5() {
    let x = {
        let y = 5;
        y + 1
    };

    println!("{x}");
}

fn sub6() {
    let x = [1,2,3];
    println!("{:?}", x);
    println!("{:#?}", x);
}

fn sub7() {
    println!("i8: {} {}", i8::MIN, i8::MAX);
}

fn sub8() {
    let x = 8;
    println!("{x}");
    {
        let x = 2.3;
        println!("{x}");
    }
    println!("{x}");
    let x = "hello";
    println!("{x}");
}