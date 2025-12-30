use tokio;

async fn get_val() -> u8 {
    8
}

#[tokio::main]
async fn main() {
    let x = get_val().await;
    println!("{}", x);
}
