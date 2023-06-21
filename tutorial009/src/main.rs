fn main() {
    

    let num = 5;

    match num {
        1 => println!("its 1"), 
        2 => println!("its 2"), 
        3 => println!("its 3"), 
        _ => println!("its other number")
    }


    let mut i = 3;
    loop {
        println!("{i:?}");
        i = i - 1;
        
        if i == 0 {
            break;
        }
    }

    let mut i = 1;
    while i <= 3 {
        println!("{i}");
        i = i + 1;
    }

    println!("Done!");

}
