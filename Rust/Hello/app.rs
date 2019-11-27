use std::io;

fn main() {
    let mut name = String::new();
    println!("Hello, what's your name? ");
    io::stdin().read_line(&mut name)
        .expect("Failed to read line.");
    println!("Hello, {}", name);
}