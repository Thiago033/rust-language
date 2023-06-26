struct LineItem {
    name: String,
    count: i32
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn main() {
    //Verctors
    let numbers = vec![10,20,30,40,50]; 

    for num in &numbers {
        match num {
            30 => println!("Thirty"),
            _ => println!("{num}"),
        }
    }

    println!("Number of elements = {:?}", numbers.len());


    //Strings
    let receipt = vec![
        LineItem {
            name: "Cereal".to_owned(),
            count: 1
        },
        LineItem {
            name: String::from("Fruit"),
            count: 3
        }
    ];

    for item in receipt {
        print_name(&item.name);
        println!("count: {:?}", item.count);
    }

}


