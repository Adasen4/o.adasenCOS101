fn main() {
    // A tuple can contain values of different types.
    let t: (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Language: {}", t.0);
    println!("Value: {}", t.1);
    println!("Score: {}", t.2);

    // Rust can also infer the types.
    let another = ("Rust", "fun", 100);
    println!("Inferred tuple: {:?}", another);
}
