async fn sub() -> u8 {
    42u8
}

fn main() {
    println!("{}", sub().await)
}
