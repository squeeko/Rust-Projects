use csv::ReaderBuilder;
use polars::prelude::*;
use std::error::Error;

// Parsing a CSV file, create a ReaderBuilder to iterate the "records"
fn read_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

// Use polars to clean the data, drop nulls and duplicates and many other things - https://docs.rs/polars/latest/polars/
fn clean_data(df: DataFrame) -> DataFrame {
    df.drop_nulls::<String>(None);
    df
}

// Use polars again to transform columns, perform aggregations and so on.
fn transform_data(mut df: DataFrame) -> Result<DataFrame, dyn Error> {
    let new_col = df.column("existing_col")?;
    df = df.hstack(&[Series::new("new_col", &[new_col])])?;
    Ok(df)
}

fn main() {}
