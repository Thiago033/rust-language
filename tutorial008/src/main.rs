fn main() {
    test_one();
    test_one();

    add_numbers(5, 5);


    let number = {
        let x = 3;
        x + 1
    };

    println!("{number}");

    let result = add_numbers_2(55, 5);
    println!("{result}");

}

fn test_one() {
    println!("Test has been called...");
}

fn add_numbers(x: i32, y: i32) {
    println!("The sum is {}", x + y);
}

//returning values
fn add_numbers_2(x: i32, y: i32) -> i32 {
    x + y
    //or
    //return x + y;

    // let result = x + y;
    // if result > 10 {
    //     return result - 10;
    // }
    
    // result
}
