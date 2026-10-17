fn main() {
    // Create an empty vector, then add values later.
    let mut numbers: Vec<i64> = Vec::new();
    numbers.push(10);
    numbers.push(20);
    println!("Numbers: {:?}", numbers);
    println!("Length: {}", numbers.len());

    // Or create a vector with values already in it.
    let names = vec!["Grace", "Effiong", "Ada"];
    println!("Names: {:?}", names);
    println!("Length: {}", names.len());
}
