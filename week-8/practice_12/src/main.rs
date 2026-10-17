fn main() {
    let mut colors = vec!["red", "green", "blue", "yellow"];

    {
        // This mutable slice borrows elements at indices 1 and 2.
        let sliced_colors = &mut colors[1..3];
        sliced_colors[1] = "purple"; // Changes colors[2]
    }

    // The original vector reflects the change made through the slice.
    println!("Updated colors: {:?}", colors);
}
