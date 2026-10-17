fn main() {
    let mut mountain_heights = ("Everest", 8848, "K2");
    mountain_heights.2 = "Lhotse";

    println!("Updated tuple: {:?}", mountain_heights);
}

