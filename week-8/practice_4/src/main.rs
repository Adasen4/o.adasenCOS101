fn main() {
    let names = vec!["Amina", "Tunde", "Grace"];
    let ages = vec![20, 22, 19];

    // Both vectors must have matching lengths when indexing them this way.
    for i in 0..ages.len() {
        println!("{} is {} years old", names[i], ages[i]);
    }
}
