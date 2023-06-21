fn main() {
    let coord: (i32, i32) = coordinate();

    if coord.1 > 5 {
        println!(">5");
    } else if  coord.1 < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}

fn coordinate() -> (i32, i32) {
    (1,7)
}

//let types = ("red", 1, 2, "blue", 3)