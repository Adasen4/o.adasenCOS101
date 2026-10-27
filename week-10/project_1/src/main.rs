struct Market {
    brand: String,
    price: u32,
}

impl Market {
    fn total(&self) -> u32 {
        self.price * 3
    }
}


fn main() {

    let product1 = Market {
        brand: String::from("HP"),
        price: 650_000
    };

    let product2 = Market {
        brand: String::from("IBM"),
        price: 755_000
    };

    let product3 = Market {
        brand: String::from("Toshiba"),
        price: 550_000
    };

    let product4 = Market {
        brand: String::from("Dell"),
        price: 850_000
    };

    println!("Since you are purchasing 3 items from each brand");
    println!("\nThe price of 1 {} is {}, making your total {}.", product1.brand, product1.price, product1.total());
    println!("The price of 1 {} is {}, making your total {}.", product2.brand, product2.price, product2.total());
    println!("The price of 1 {} is {}, making your total {}.", product3.brand, product3.price, product3.total());
    println!("The price of 1 {} is {}, making your total {}.", product4.brand, product4.price, product4.total());

    let sum_total = product1.total() + product2.total() + product3.total() + product4.total();
    println!("\nMaking the cost of your entire order {}", sum_total);

}