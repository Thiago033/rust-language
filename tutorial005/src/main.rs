use std::io;

fn main() {
    
    println!("Type something: ");

    let mut input: String = String::new();
    
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);
}
