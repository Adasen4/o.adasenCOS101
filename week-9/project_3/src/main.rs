use std::fs::File;
use std::io::Write;

fn main() {
    let comm = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Ozazuwa Faith Etiye",
    ];

    let ministry = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zone = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file = File::create("efcc_merge.csv").unwrap();
    writeln!(file, "Commissioner,Ministry,Zone").unwrap();

    for i in 0..comm.len() {
        writeln!(file, "{},{},{}", comm[i], ministry[i], zone[i]).unwrap();
    }

    println!("Data has been sent to file");
}