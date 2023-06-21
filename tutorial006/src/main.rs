use std::io;

fn main() {
    // let _x: u8 = 9;  // 0 - 255
    // let _y: i8 = 10; // -128 - 127 

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Expected to read a line");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{int_input}");
    
}
