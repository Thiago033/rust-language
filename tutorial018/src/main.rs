struct Sedan;

impl LandCapable for Sedan {
    fn drive(&self) {
        println!("Sedan in driving");
    }
}

struct Suv;

impl LandCapable for Suv {
    fn drive(&self) {
        println!("Suv in driving");
    }
}

trait LandCapable {
    fn drive(&self) {
        println!("Default driving");
    }
}

trait WaterCapable {
    fn float(&self) {
        println!("Default float");
    }
    
}

trait Amphibious : WaterCapable + LandCapable{ }

struct Hovercraft;
impl Amphibious for Hovercraft{}
impl LandCapable for Hovercraft{
    fn drive(&self) {
        println!("Hovercraft diving");
    }
}
impl WaterCapable for Hovercraft{}


fn road_trip(vehicle: &impl LandCapable) {
    vehicle.drive();
}

fn traverse_frozen_lake(vehicle: &impl Amphibious) {
    vehicle.drive();
    vehicle.float();
}

fn main() {
    let car_sedan = Sedan;
    road_trip(&car_sedan);

    let car_suv = Suv;
    road_trip(&car_suv);

    let hc = Hovercraft;
    traverse_frozen_lake(&hc);
}
