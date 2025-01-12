#[allow(dead_code)]
struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn calculate_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    let total_cost = hp.calculate_cost(3)
        + ibm.calculate_cost(3)
        + toshiba.calculate_cost(3)
        + dell.calculate_cost(3);

    println!("The total cost for purchasing 3 laptops from each brand is: ₦{}", total_cost);
}
