// Using std::fs and std::io for reading and writing of files
use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // reads the entire contents of a file into a string.
    // pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String>
    Ok(contents)
}

// Using crates for structured data CSV and JSON
use csv::Reader;

fn read_csv_data(path: &str) -> csv::Result<()> {
    let mut rdr = Reader::from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    match read_file_contents("./data/data-shortened.csv") {
        Ok(data) => println!("File data: {}", data),
        Err(e) => println!("Failed to read file: {}", e),
    }

    // Success sample -
    // 4718-WXBGI,Male,0,Yes,Yes,72,Yes,Yes,DSL,Yes,Yes,Yes,Yes,Yes,Yes,Two year,Yes,Credit card (automatic),91.95,6614.9,No
    // 2867-UIMSS,Male,0,No,No,1,Yes,No,Fiber optic,No,No,No,No,No,Yes,Month-to-month,No,Electronic check,80.5,80.5,Yes
    // 8495-PRWFH,Female,1,No,No,42,Yes,Yes,DSL,No,No,Yes,No,No,No,Month-to-month,No,Electronic check,55.65,2421.75,No

    match read_csv_data("./data2/data-shortened.csv") {
        Ok(record) => println!("Reading csv record: {:?}", record),
        Err(e) => println!("Failed to read csv record: {}", e),
    }
}
