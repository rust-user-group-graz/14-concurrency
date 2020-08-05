use smol::io;

async fn sub() -> u8 {
  42u8
}


fn main() -> io::Result<()> {
    smol::run(async {
        println!("{}", sub().await);
        Ok(())
    })
}
