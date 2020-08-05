use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mutex = Arc::new(Mutex::new(0));
    let c_mutex = mutex.clone();

    thread::spawn(move || {
        *c_mutex.lock().unwrap() = 10;
    }).join().expect("thread::spawn failed");
    println!("{}", *mutex.lock().unwrap());
}
