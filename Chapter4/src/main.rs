fn main() {
    sub4_1();
    sub4_1_3();
    sub4_1_5();
    sub4_2_1();
    sub4_3_1();
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

fn time_to_thing2(time_hour: i32) -> Things2 {
    use Things2::*;
    match time_hour {
        6..=18 => Sun(String::from("sun")),
        _ => Things2::Stars(7),
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

    use Things2::*;
    match time_to_thing(12) {
        Sun(s) => println!("{s}"),
        Stars(n) => println!("stars: {n}"),
    }

    match time_to_thing2(3) {
        Sun(s) => println!("{s}"),
        Stars(n) => println!("stars: {n}"),
    }
}

enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

fn sub4_1_3() {
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("{}", season as u32);
    }
}


#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

#[derive(Debug)]
struct Animal {
    age: u8,
    atype: AnimalType,
}

impl Animal {
    fn new_cat() -> Self {
        Self {
            age: 0,
            atype: AnimalType::Cat,
        }
    }

    fn to_dog(&mut self) {
        self.atype = AnimalType::Dog;
    }
    fn to_cat(&mut self) {
        self.atype = AnimalType::Cat;
    }
    fn age(&mut self, extra: u8) {
        self.age += extra;
    }
}

fn sub4_1_5() {
    let mut x = Animal::new_cat();
    println!("{:?}", x);
    x.to_dog();
    x.age(10);
    println!("{:?}", x);
}


struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

fn sub4_2_1() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false
    };
 
    let Person {
        name: fake_name,
        real_name,
        height,
        happiness,
    } = papa_doc;
 
    println!("They call him {fake_name} but his real name is {real_name}. 
    He is {height} cm tall and is he happy? {happiness}");

    let x = City::new("Tallinn", "Reval", 426_538, 1219);
    x.print_names();

}

struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}

impl City {
    fn new(name: &str, name_before: &str, population: u32, date_founded: u32) -> Self {
        Self {
            name: String::from(name),
            name_before: String::from(name_before),
            population: population,
            date_founded: date_founded,
        }
    }

    fn print_names(&self) {
        let City {
            name, name_before, ..
        } = self;
        println!("The city {name} used to be called {name_before}");
    }
}


fn sub4_3_1() {
    let my_name = "Billy".to_string();
    println!("{}", my_name.is_empty());
    println!("{}", &&&&&&my_name.is_empty());

    let y = "Billy".to_string();
    println!("{}", my_name == y);


}