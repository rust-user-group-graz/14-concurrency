fn foo(a: &[u8], b: &[u8], c: &mut [u8]) {
    for ((a, b), c) in a.iter().zip(b).zip(c) {
        println!("{} {} {}", a, b, c);
        *c = *a + *b;
    }
}

fn main() {
    let a: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    let b: [u8; 4] = [0x00, 0x01, 0x02, 0x03];
    let mut c = [0u8; 4];
    foo(&a, &b, &mut c);

    println!("{:?}", c);
}
