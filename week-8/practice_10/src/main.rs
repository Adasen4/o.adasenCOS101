fn main() {
    let x: (i32, bool, f64) = (21, true, 3.75);
    let (age, is_male, cgpa) = x;

    println!("Age is {}, male: {}, CGPA: {}", age, is_male, cgpa);
}
