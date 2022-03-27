trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16
}

struct Car {
    info: VehicleInfo
}

impl Park for Car {
    fn park(&self) {
        println!("parking car!");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo
}

impl Truck {
    fn unload(&self) {
        println!("unloading truck.")
    }
}

impl Park for Truck {
    fn park(&self) {
        println!("parking truck!");
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house: {}", color);
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995
        }
    };
    let house = House {};
    let object = create_paintable_object();

    paint_red(&car);
    paint_red(&house);
    paint_red(&object);

    paint_vehicle_red(&car);
}

fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_owned());
}

fn paint_vehicle_red<T>(object: &T) where T: Paint + Park {
    object.paint("red".to_owned());
}

fn create_paintable_object() -> impl Paint {
    House {}
}