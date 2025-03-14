enum FuelType {
    Petrol,
    Diesel,
    Electric,
}

struct Vehicle {
    brand: String,
    model: String,
    fuel_type: FuelType,
}

fn filter_electric_vehicles(vehicles: &Vec<Vehicle>) {
    for vehicle in vehicles {
        if let FuelType::Electric = vehicle.fuel_type {
            println!("Brand: {}, Model: {}", vehicle.brand, vehicle.model);
        }
    }
}

fn main() {
    let vehicles = vec![
        Vehicle {
            brand: "Tesla".to_string(),
            model: "Model S".to_string(),
            fuel_type: FuelType::Electric,
        },
        Vehicle {
            brand: "Toyota".to_string(),
            model: "Corolla".to_string(),
            fuel_type: FuelType::Petrol,
        },
        Vehicle {
            brand: "Nissan".to_string(),
            model: "Leaf".to_string(),
            fuel_type: FuelType::Electric,
        },
    ];
    println!("Electric Vehicles:");
    filter_electric_vehicles(&vehicles);
}