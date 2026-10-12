use std::io;

fn error1(){
    println!("You have entered a wrong input");
    println!("Remember enter either O, A, L or T");
}

fn error2(){
    println!("You have entered a wrong input");
    println!("Remember enter a number from 0 to 5");
}

fn error3(){
    println!("You have entered a wrong input");
    println!("Remember enter a number from 1 to 5");
}

fn main() {

    let pservice = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    println!("Welcome to this APL Level Checker App");
    println!("To begin input enter your field of choice: ");

    println!("Enter O if you are an office admin");
    println!("Enter A if you are an acadamian");
    println!("Enter L if you are a lawyer");
    println!("Enter T if you are a teacher");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let category = input1.trim().to_uppercase();

    if category == "O" {
        println!("\nGreat, you are an office administrator");
        println!("Now let's determine your level in the public service");
        let mut level1 = String::new();

        println!("\nEnter 0 if your an intern");
        println!("Enter 1 if your an administrator");
        println!("Enter 2 if your a senior administrator");
        println!("Enter 3 if your an office manager");
        println!("Enter 4 if your a director");
        println!("Enter 5 if your a CEO");

        io::stdin().read_line(&mut level1).unwrap();
        let index1:usize = level1.trim().parse().unwrap();
        if index1>5 {
            error2();
            return;
        }

        println!("\nIn the Public Service you are at level {}", pservice[index1]);
    }

    else if category == "A" {
        println!("\nGreat, you are an acadamian");
        println!("Now let's determine your level in the public service");
        let mut level2 = String::new();

        println!("\nEnter 1 if your a Research assistant");
        println!("Enter 2 if your a PhD Candidate");
        println!("Enter 3 if your a Post-Doc Researcher");
        println!("Enter 4 if your a Senior Lecturer");
        println!("Enter 5 if your a Dean");

        io::stdin().read_line(&mut level2).unwrap();
        let index2:usize = level2.trim().parse().unwrap();
        if index2<1 || index2>5 {
            error3();
            return;
        }

        println!("\nIn the Public Service you are at level {}", pservice[index2]);
    } 

    else if category == "L" {
        println!("\nGreat, you are a lawyer");
        println!("Now let's determine your level in the public service");
        let mut level3 = String::new();

        println!("\nEnter 0 if your a Paralegal");
        println!("Enter 1 if your a Junior Associate");
        println!("Enter 2 if your a Associate");
        println!("Enter 3 if your a Senior Associate 1 or 2");
        println!("Enter 4 if your a Senior Associate 3 or 4");
        println!("Enter 5 if your a Partner");

        io::stdin().read_line(&mut level3).unwrap();
        let index3:usize = level3.trim().parse().unwrap();
        if index3>5 {
            error2();
            return;
        }

        println!("\nIn the Public Service you are at level {}", pservice[index3]);
    } 

    else if category == "T" {
        println!("\nGreat, you are a Teacher");
        println!("Remember, teachers are the backbone of a nation");
        println!("\nNow let's determine your level in the public service");
        let mut level5 = String::new();

        println!("\nEnter 0 if your a Placement");
        println!("Enter 1 if your a Classroom Teacher");
        println!("Enter 2 if your a Senior Teacher");
        println!("Enter 3 if your a Leading Teacher");
        println!("Enter 4 if your a Deputy Principal");
        println!("Enter 5 if your a Principal");

        io::stdin().read_line(&mut level5).unwrap();
        let index5:usize = level5.trim().parse().unwrap();
        if index5>5 {
            error2();
            return;
        }

        println!("\nIn the Public Service you are at level {}", pservice[index5]);
    }

    else {
        error1();
    }
}
