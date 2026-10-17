fn print_person(x: (i32, bool, f64)) {
    println!("Person tuple: {:?}", x);
}

fn main() {
    let person = (21, true, 3.75);
    print_person(person);
}
