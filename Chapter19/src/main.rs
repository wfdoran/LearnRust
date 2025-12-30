use tokio;
use rand::*;
use tokio::join;
use std::time::Duration;

async fn get_val() -> u8 {
    8
}

#[tokio::main]
async fn main() {
    let x = get_val().await;
    println!("{}", x);

    get_many().await;

}

async fn get_val2(num: u8) -> u8 {
    let mut rng = rand::rng();
    let wait_time = rng.random_range(1..100);
    tokio::time::sleep(Duration::from_millis(wait_time)).await;
    println!("got {}", num);
    num
}

async fn get_many() {
    let nums = join!(
        get_val2(1), 
        get_val2(2),
        get_val2(3)
    );

    println!("{nums:?}");
}
