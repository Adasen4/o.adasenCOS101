fn main() {
    let letters = vec!['R', 'u', 's', 't'];
    let index: usize = 8;

    // get() returns Some(value) when the index exists, or None otherwise.
    let ch = letters.get(index);
    println!("Result: {:?}", ch);
}

