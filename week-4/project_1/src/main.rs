use std::io;

fn main() {
    println!("Welcome to your quadratic equation calculator");

    println!("\nEnter your value of a");
    let mut input1= String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f64 = input1.trim().parse().expect("Failed to read input");

    println!("\nEnter your value of b");
    let mut input2= String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f64 = input2.trim().parse().expect("Failed to read input");

    println!("\nEnter your value of c");
    let mut input3= String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f64 = input3.trim().parse().expect("Failed to read input");   
    
   
    let d:f64 = b*b - 4.0*a*c;

    if d>0.0 {
        println!("The quadratic equation has two distinct roots");
    } 

    if d==0.0 {
        println!("The quadratic equation has exactly one real root");
    }

    if d<0.0 {
        println!("The quadratic equation has no real roots");
    }
}
