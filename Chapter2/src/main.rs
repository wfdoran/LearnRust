fn main() {
    sub2_1();
    sub2_2();
    sub2_3();
    sub2_4();
    sub2_5();
    sub2_6();
}

fn sub2_1() {
    let n = 15;
    let x = &n;
    let y = &x;

    println!("{}", y);
}

fn sub2_2() {
    let x = "hello, world!";
    let y = String::from(x);
    let z = x.to_string();

    println!("{}; {}; {}", x, y, z);

    let t1 = std::mem::size_of::<String>();
    let t2 = std::mem::size_of::<i8>();
    let t3 = std::mem::size_of::<f64>();

    println!("{} {} {}", t1, t2, t3);

    let s1 = std::mem::size_of_val(&5.3);
    let s2 = std::mem::size_of_val(x);
    let s3 = std::mem::size_of_val(&y);

    println!("{} {} {}", s1, s2, s3);

    let age = 50;
    let name = "Wild Bill";
    let r = format!("{name} is {age} years old");
    println!("{r}");

    let g: String = 'x'.into();
    println!("{g}");
}


const MM_PER_INCH: f64 = 25.4;
static FIB: [i32; 8] = [1,1,2,3,5,8,13,21];
fn sub2_3() {
    println!("{}", 3.2 * MM_PER_INCH);
    println!("{}", FIB[4]);
}

fn return_str() -> String {
    let country = String::from("Mexico");
    country
}

fn sub2_4() {
    let country = String::from("Canada");
    let ref_one = &country;
    let ref_two = &country;
    println!("{}", ref_one);


    let q = return_str();
    println!("{}", q);

    println!("{}", ref_two);
}

fn sub2_5() {
    let mut n = 4;
    let r = &mut n;
    println!("{}", r);
    // n = 5;
    *r += 5;
    println!("{}", r);

    let q = &mut n;

    println!("{}", q);

    n = 100;
    println!("{}", n);
    
    {
        let mut number = 10;
        let number_change = &mut number;
        *number_change += 10;
        let number_ref = &number;
        println!("{}", number_ref);
    }
}


fn sub2_6() {
    {
        let country = String::from("Austria");
        let country_ref = &country;
        let country = 8;
        println!("{country_ref} {country}");
    }

}