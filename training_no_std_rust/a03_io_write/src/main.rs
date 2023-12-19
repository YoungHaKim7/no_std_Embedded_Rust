use std::io::Write;

fn main() {
    let x = 123;
    let mut buf = [0 as u8; 20];
    write!(&mut buf[..], "{}", x).expect("Can't write");
    assert_eq!(&buf[0..3], b"123");
}
