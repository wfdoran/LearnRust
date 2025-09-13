fn main() {
    sub3_1_1();
    sub3_1_2();
    sub3_1_3();
    sub3_2();
}

fn sub3_1_1() {
    let a1 = ["one", "two", "three"];
    println!("{:?}", a1);
    println!("{}",a1[1]);

    let mut b = [0u8; 5];
    println!("{:?}", b);
    b[2] = 3;
    println!("{:?}", b);

    let c = [0,1,2,3,4,5,6,7,8,9];
    let d = &c[3..7];
    println!("{:?}", d);

    let mut e = [1,2,3,4,5];
    let f = &mut e[1..4];
    f[0] = 10;
    println!("{:?}", e);
}

fn sub3_1_2() {
    let a = String::from("Hello ");
    let b = String::from("World");
    let mut c = Vec::new();
    c.push(a);
    c.push(b);
    println!("{:?}", c);


    let mut d: Vec<(i32,i32)> = Vec::new();
    d.push((1,2));
    d.push((3,4));
    println!("{:?}", d);

    println!("{}", d.capacity());
    d.push((5,2));
    d.push((6,4));
    d.push((7,2));
    d.push((8,4));

    println!("{} {}", d.len(), d.capacity());


    let e = [1,2,3,4,5];
    let f:Vec<_> = e.into();
    println!("{:?} {} {}", f, f.len(), f.capacity());
   
}

fn sub3_1_3() {
    let random_tuple = ("string", 8, 4.3, vec!['a'], [7,8,9]);
    println!("{:?}", random_tuple);
    println!("{}", random_tuple.1);

    let (_, _, a, _, b) = random_tuple;
    println!("{}", a);
    println!("{:?}", b);

}

fn sub3_2() {
    let x = 5;
    if x == 7 {
        println!("yes");
    } else {
        println!("no");
    }

    let y: u8 = 5;
    match y {
        0 | 1 | 2 => println!("really small"),
        4 | 5 | 6 => println!("middle"),
        _ => println!("big!"),
    }

    let z = match y {
        0 => 0,
        1 => 1,
        2 => 4,
        3 => 9,
        4 => 16,
        5 => 25,
        _ => -1,
    };
    println!("{z}");

    match (x, z) {
        (x,z) if x > 0 && z >= 0 => println!("both positive"),
        (x,_) if x >= 0 => println!("only x"),
        (_,z) if z >= 0 => println!("only z"),
        _ => println!("neither"),
    };

    match y {
        y @ 0..=127 => println!("{y} is low"),
        y @ 128..=255 => println!("{y} is hi"),
    }
}