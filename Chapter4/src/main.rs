fn main() {
    sub4_1();
}

#[derive(Debug)]
struct ColorRGB(u8, u8, u8);

struct SizeAndColor {
    size: u32,
    color: ColorRGB,
}

#[derive(Debug)]
enum Climate {
    Tropical, Dry, Temporate, Continental, Polar,
}

#[derive(Debug)]
struct Country {
    population: i64,
    capital: String,
    leader: String,
    climate: Climate,
}

#[derive(Debug)]
enum ThingsInTheSky {
    Sun, Stars,
}

fn time_to_sky(time_hour: i32) -> ThingsInTheSky {
    match time_hour {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars,
    }
}

enum Things2 {
    Sun(String),
    Stars(i32),
}

fn time_to_thing(time_hour: i32) -> Things2 {
    match time_hour {
        6..=18 => Things2::Sun(String::from("sun")),
        _ => Things2::Stars(5),
    }
}

fn sub4_1() {
    let red = ColorRGB(255, 0, 0);
    let x = SizeAndColor {
        size: 3847474,
        color: red,
    };
    println!("{} {}", x.size, x.color.0);

    let color = ColorRGB(128, 128, 128);
    let y = SizeAndColor {
        size: 3838,
        color,
    };

    println!("{:?}", y.color);

    let z = Country {
        population: 5_000_000,
        capital: String::from("Ottawa"),
        leader: String::from("Mark Carney"),
        climate: Climate::Polar,
    };
    println!("{:?}", z);

    println!("{:?} {:?}", time_to_sky(12), time_to_sky(20));

    match time_to_thing(12) {
        Things2::Sun(s) => println!("{s}"),
        Things2::Stars(n) => println!("stars: {n}"),
    }
}