fn main() {
    println!("Hello, world!");
    let immutable = 10;
    println!("immutable is {}", immutable);

    let mut mutable = 50;
    println!("[Before] mutable is {}", mutable);
    mutable = 60;
    println!("[After] mutable is {}", mutable);
}
