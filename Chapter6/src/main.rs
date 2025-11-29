use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::time::Instant;

struct City {
    name: String,
    population: HashMap<i32, i32>,
}

struct City2 {
    name: String,
    population: BTreeMap<i32, i32>,
}


fn chap6_1_1() {
    let mut san_deigo = City {
        name: "San Diego".to_string(),
        population: HashMap::new(),
    };

    san_deigo.population.insert(1950, 440_000);
    san_deigo.population.insert(2000, 2_681_000);
    san_deigo.population.insert(2025, 3_373_000);

    for (y,p) in san_deigo.population {
        println!("In {y}, {:?} had a population of {p}.", san_deigo.name);
    }
    println!("");
}

fn chap6_1_2() {
    let mut san_deigo = City2 {
        name: "San Diego".to_string(),
        population: BTreeMap::new(),
    };

    san_deigo.population.insert(1950, 440_000);
    san_deigo.population.insert(2000, 2_681_000);
    san_deigo.population.insert(2025, 3_373_000);

    for (y,p) in san_deigo.population {
        println!("In {y}, {:?} had a population of {p}.", san_deigo.name);
    }
    println!("");
}

fn chap6_1_3() {
    let mut a = HashMap::new();
    a.insert("foo", "bar");

    println!("{:?}", a["foo"]);
    println!("{:?}", a.get("foo"));
    println!("{:?}", a.get("fooo"));

    match a.get("foo") {
        Some(v) => println!("{v}"),
        None => println!("nope"),
    }
}

fn chap6_1_4() {
    let mut a = HashMap::new();

    a.insert("a", false);

    let mut b = vec!["a", "b", "c", "c"];

    for x in &b {
        a.entry(x).or_insert(true);
    }

    for (x,y) in a {
        println!("{:?} {y}", x);
    }

    b.remove(1);
    println!("{:?}", b);
}

fn ver1() {
    let mut a = vec![0; 600_000];
    for _ in 0..600_000 {
        a.remove(0);
    }
}

fn ver2() {
    let mut a = VecDeque::from(vec![0; 600_000]);
    for  _ in 0 ..600_000 {
        a.pop_front();
    }
}

fn chap6_1_5() {
    let start1 = Instant::now();
    ver1();
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    ver2();
    let duration2 = start2.elapsed();

    println!("{:?} -- {:?}", duration1, duration2);
}


fn main() {
    chap6_1_1();
    chap6_1_2();
    chap6_1_3();
    chap6_1_4();
    chap6_1_5();
}