use std::env;
use std::error::Error;
use std::fs::File;
use std::process;

use csv::ReaderBuilder;

fn read_csv_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    for result in csv_reader.records() {
        let record = result?;
        for field in record.iter() {
            println!("{}", field);
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <csv_file_path>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    if let Err(err) = read_csv_file(file_path) {
        eprintln!("Error reading CSV file: {}", err);
        process::exit(1);
    }
}
