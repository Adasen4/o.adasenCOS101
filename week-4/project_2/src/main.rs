use std::io;

fn main() {

    println!("Enter your age");
    let mut input1= String::new();
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let age:u64 = input1.trim().parse().expect("Invalid input");

    println!("Enter how many years you've worked for");
    let mut input2= String::new();
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let worked:u64 = input2.trim().parse().expect("Invalid input");
    //years worked

    //I'm using 10 years of working as the experience benchmark
    if worked>=10 {
        let level1:&str = "experienced";
        println!("You are {}", level1);
    }

    if age>=40 && worked>=10 {
        println!("Your annual incentive is N1,560,000");
        println!("aka 1.56 million naira");
    }

    else if age>=30 && age<=39 && worked>=10 {
        println!("Your annual incentive is N1,480,000");
        println!("aka 1.48 million naira");
    }

    else if age<=29 {
        println!("Your annual incentive is N1,300,000");
        println!("aka 1.3 million naira");
    }

    else{
        println!("You are not experienced");
        println!("Your annual incentive is N100,000");
    }
}
