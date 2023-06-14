fn main() {

    // Integer:

    // default i32 - signed integer
    let x = 2;

    // u32 - UNsigned integer
    let y: u32 = 2;

    // i8
    // i16
    // i32
    // i64
    // i128


    // Float:

    // default f32 - signed float
    let floating_point = 10.9;

    // f8
    // f16
    // f32
    // f64
    // f128


    // Bool
    let bool_val: bool = true;

    // Char
    let letter: char = 'a';

    // Tuples
    let tup: (i32, bool, char) = (1, true, 's'); //type default: (i32, bool, char)
    println!("{}", tup.0);

    let mut tup2: (i8, bool, char) = (1, false, 'a');
    tup2.0 = 10;
    println!("{}", tup2.0);

    // Array
    let mut arr = [1,2,3,4,5];
    arr[4] = 10;
    println!("{}", arr[4])
    

}