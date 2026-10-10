use std::io;

fn trap(height1:f64, base1:f64 , base2:f64) {
    let area1:f64 = (height1/2.0)*(base1 + base2);
    println!("\nThe area of your trapezium is {}",area1);
}

fn rhomb(diag1:f64, diag2:f64) {
    let area2:f64 = 0.5*diag1*diag2;
    println!("\nThe area of your rhombus is {}",area2);
}

fn para(base:f64, alt:f64) {
    let area3:f64 = base*alt;
    println!("\nThe area of your parallelogram is {}",area3);
}

fn cube(side:f64) {
    let area4:f64 = 6.0*side*side;
    println!("\nThe surface area of your cube is {}",area4);
}

fn cylinder(radius:f64, height2:f64) {
    let pi:f64 = 22.0/7.0;
    let vol1:f64 = pi * radius * radius * height2;
    println!("\nThe volume of your cylinder is {}", vol1);
}

fn main(){
    println!("Welcome to this Area/Volume calculator");
    // Display the list
    println!("======== SHAPE MENU ========");
    println!("T - Area of Trapezium ");
    println!("R - Area of Rhombus   ");
    println!("P - Area of Parallelogram");
    println!("A - Surface area of Cube ");
    println!("B - Volume of cylinder  ");
    println!("=================================");

    let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let finput = input.trim().to_uppercase();

    if finput == "T" {
        let mut tinput1 = String::new();
        println!("Enter the height of your trapezium: ");
        io::stdin().read_line(&mut tinput1).expect("Failed to read input");
        let height1:f64 = tinput1.trim().parse().expect("Invalid input");

        let mut tinput2 = String::new();
        println!("Enter the 1st base of your trapezium: ");
        io::stdin().read_line(&mut tinput2).expect("Failed to read input");
        let base1:f64 = tinput2.trim().parse().expect("Invalid input");

        let mut tinput3 = String::new();
        println!("Enter the 2nd base of your trapezium: ");
        io::stdin().read_line(&mut tinput3).expect("Failed to read input");
        let base2:f64 = tinput3.trim().parse().expect("Invalid input");

        trap(height1, base1, base2);
    }

    if finput == "R" {
        let mut rinput1 = String::new();
        println!("Enter the 1st diagonal of your rhombus: ");
        io::stdin().read_line(&mut rinput1).expect("Failed to read input");
        let diag1:f64 = rinput1.trim().parse().expect("Invalid input");

        let mut rinput2 = String::new();
        println!("Enter the 2nd diagonal of your rhombus: ");
        io::stdin().read_line(&mut rinput2).expect("Failed to read input");
        let diag2:f64 = rinput2.trim().parse().expect("Invalid input");

        rhomb(diag1, diag2);
    }

    if finput == "P" {
        let mut pinput1 = String::new();
        println!("Enter the base of your parallelogram: ");
        io::stdin().read_line(&mut pinput1).expect("Failed to read input");
        let base:f64 = pinput1.trim().parse().expect("Invalid input");

        let mut pinput2 = String::new();
        println!("Enter the altitude of your parallelogram: ");
        io::stdin().read_line(&mut pinput2).expect("Failed to read input");
        let alt:f64 = pinput2.trim().parse().expect("Invalid input");

        para(base, alt);
    }

    if finput == "A" {
        let mut ainput1 = String::new();
        println!("Enter the length of the side of the cube: ");
        io::stdin().read_line(&mut ainput1).expect("Failed to read input");
        let side:f64 = ainput1.trim().parse().expect("Invalid input");

        cube(side);
    }

    if finput == "B" {
        let mut binput1 = String::new();
        println!("Enter the radius of your cylinder: ");
        io::stdin().read_line(&mut binput1).expect("Failed to read input");
        let radius:f64 = binput1.trim().parse().expect("Invalid input");

        let mut binput2 = String::new();
        println!("Enter the height of your cylinder: ");
        io::stdin().read_line(&mut binput2).expect("Failed to read input");
        let height2:f64 = binput2.trim().parse().expect("Invalid input");

        cylinder(radius, height2);
    }

    println!("Thank you for using this calculator");
    println!("Have a nice day");
}
