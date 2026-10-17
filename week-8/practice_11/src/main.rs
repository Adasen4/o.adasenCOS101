fn main() {
    let numbers = [10, 20, 30, 40, 50];

    // Slice ranges include the start index and exclude the end index.
    let slice1 = &numbers[1..3]; // Elements at indices 1 and 2
    let slice2 = &numbers[..3];  // From the beginning up to index 3
    let slice3 = &numbers[1..];  // From index 1 to the end
    let slice4 = &numbers[..];   // The whole array

    println!("[1..3] = {:?}", slice1);
    println!("[..3] = {:?}", slice2);
    println!("[1..] = {:?}", slice3);
    println!("[..] = {:?}", slice4);
}
