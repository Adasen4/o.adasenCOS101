use std::fs::File;
use std::io::Write;

fn main() {
    let names = vec![
        "Oluchi Mordi",
        "Oghenegare Adasen",
        "Tomiwa Atiba",
        "Adekunle Gold",
        "Jamal Edemoh",
    ];

    let matric = vec![
        "ACC10211111",
        "ECO10110101",
        "CSC10328828",
        "EEE11020202",
        "MEE10202001",
    ];

    let dept = vec![
        "Accounting",
        "Economics",
        "Computer Science",
        "Electrical",
        "Mechanical",
    ];

    let level = vec![
        "300",
        "100",
        "200",
        "200",
        "100",
    ];

   // Column widths
    let w1 = 20;
    let w2 = 16;
    let w3 = 15;

    // Display on the terminal
    println!("\n{:w1$} {:w2$} {:w3$} {}", "Student Name", "Matric. Number", "Department", "Level");

    for i in 0..names.len() {
        println!("\n{:w1$} {:w2$} {:w3$} {}", names[i], matric[i], dept[i], level[i]);
    }

    let mut file = File::create("PAU_SMIS.csv").unwrap();
    writeln!(file, "Student Name,Matric. Number,Department,Level").unwrap();

    for i in 0..names.len() {
        writeln!(file, "{},{},{},{}", names[i], matric[i], dept[i], level[i]).unwrap();
    }

    println!("\nData has been sent to a .csv file");
}