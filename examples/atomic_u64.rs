use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    let uint = AtomicU64::new(41);
    uint.store(42, Ordering::SeqCst);
    println!("{}", uint.load(Ordering::SeqCst));
}
