struct Laptop {
    brand: String,
    price: u32,
}
impl Laptop {
    fn calculate_cost(&self, quantity: u32) ->u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };

    let ibm = Laptop{
        brand: String::from("IBM"),
        price: 755_000,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };

    let dell = Laptop{
        brand: String::from("Dell"),
        price: 850_000,
    };

    let qty = 3;
    let hp_total = hp.calculate_cost(qty);
    let ibm_total = ibm.calculate_cost(qty);
    let toshiba_total = toshiba.calculate_cost(qty);
    let dell_total = dell.calculate_cost(qty);

    let grand_total = hp_total + ibm_total + toshiba_total + dell_total;

    println!("ALABA ELECTRONICS PRUCHASES");
    println!("{} HP laptops: #{}", qty, hp_total);
    println!("{} IBM laptops: #{}", qty, ibm_total);
    println!("{} Toshiba laptops: #{}", qty, toshiba_total);
    println!("{} Dell laptops: #{}", qty, dell_total);
    println!("Total cost across all brands: #{}", grand_total);
}
