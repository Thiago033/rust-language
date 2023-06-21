fn main() {
    let sparkling_drink = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 10.0,
    };
    
    let sweet_drink = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 12.0,
    };

    let fruity_drink = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 15.5,
    };

    print_drink(sparkling_drink);
    print_drink(sweet_drink);
    print_drink(fruity_drink);

}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

fn print_drink (drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("Flavor is: sparkling"),
        Flavor::Sweet => println!("Flavor is: sweet"),
        Flavor::Fruity => println!("Flavor is: fruity"),
    }

    println!("Oz: {:?}", drink.fluid_oz);
}