struct Dimensions {
    height: f64,
    width: f64,
    depth: f64
}

impl Dimensions {
    fn print(&self) {
        println!("Height: {:?}", self.height);
        println!("Width: {:?}", self.width);
        println!("Depth: {:?}", self.depth);
    }
}

enum Color {
    Black,
    Green
}

impl Color {
    fn print(&self) {
        match self {
            Color::Black => println!("Black"),
            Color::Green => println!("Green")
        }
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    color: Color,
    weight: f64
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self { 
            weight, 
            color, 
            dimensions
        }
    }

    fn print(&self) {
        self.dimensions.print();
        println!("Weight: {:?}", self.weight);
        self.color.print();
        
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0
    };

    let big_dimensions = Dimensions {
        width: 10.0,
        height: 30.0,
        depth: 8.0
    };

    let small_box = ShippingBox::new(5.0, Color::Black, small_dimensions);
    let big_box = ShippingBox::new(68.0, Color::Green, big_dimensions);

    small_box.print();
    big_box.print();
}
