fn main() {
    sub3_1_1();
    sub3_1_2();
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

