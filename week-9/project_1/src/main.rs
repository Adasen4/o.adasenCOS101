use std::io::Write;

fn main() {
    
    let heading = "Lager,Stout,Non-Alcoholic";
    let row1 = "\n33 Export,Legend,Maltina";
    let row2 = "\nDesperados,Turbo King,Amstel Malta";
    let row3 = "\nGoldberg,Williams,Malta Gold";
    let row4 = "\nGulder,,Fayrouz";
    let row5 = "\nHeineken";
    let row6 = "\nStar";

    let mut file = std::fs::File::create("Nigerian_Breweries.csv").expect("create failed");
    file.write(heading.as_bytes()).expect("write failed");
    file.write(row1.as_bytes()).expect("write failed");
    file.write(row2.as_bytes()).expect("write failed");
    file.write(row3.as_bytes()).expect("write failed");
    file.write(row4.as_bytes()).expect("write failed");
    file.write(row5.as_bytes()).expect("write failed");
    file.write(row6.as_bytes()).expect("write failed");

    println!("\nData written to file.");
}