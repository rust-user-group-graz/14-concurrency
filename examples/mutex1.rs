use std::sync::Mutex;

fn main() {
    let val = Mutex::new(0u8);
    {
        *val.lock().unwrap() += 1;
        *val.lock().unwrap() += 1;
        *val.lock().unwrap() += 1;
    }
    println!("{}", val.lock().unwrap());
}
