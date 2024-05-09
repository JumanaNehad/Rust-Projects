//to run : cargo run
//have to put the csv file we want
#![deny(clippy::all)]
use csv::Reader;
use std::error::Error;

fn main() {
    //for printing errors => eprintln
    if let Err(e) = read_from_file("./customers.csv") {
        eprintln!("{}", e);
    }
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    //read csv file
    //I want to loop over file so the variable has to be mutable
    //? instead of match
    let mut reader = Reader::from_path(path)?;

    //doesnt work with iter or alone
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record)
    }
    Ok(())
}
