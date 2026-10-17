fn main() {
    let letters = vec!['R', 'u', 's', 't'];
    let index: usize = 2;

    // Check the index first: indexing out of bounds panics.
    if index < letters.len() {
        let ch: char = letters[index];
        println!("Character at index {}: {}", index, ch);
    } else {
        println!("Index {} is outside the vector", index);
    }
}
