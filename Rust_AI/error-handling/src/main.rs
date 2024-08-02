/*
The language's `Option` and `Result` types are the
cornerstones of this philosophy, offering a clear and explicit way to handle
the possibility of absence and failure.
The `Option` type in Rust encapsulates the very idea of optionality—
expressing the potential absence of a value without resorting to null
references. It is a powerful tool in a data scientist's arsenal, used to handle
situations where data may or may not be present:
*/

use std::fs::File;
use std::io::{self, Read};

#[allow(dead_code)]

fn find_max(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        None
    } else {
        Some(data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)))
    }
}

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    /*

    In this snippet, the `find_max` function returns an `Option<f64>`, which
    can be either `Some(value)` if the dataset is not empty, or `None` if it is.

    match find_max(&dataset) {
        Some(max_value) => println!("Maximum value: {}", max_value),
        None => println!("Dataset is empty!"),
    }
    */

    // Result handles operations that can fail using Ok(T) and Err(E)

    /*
        `read_file_contents` tries to read the entire contents of a file into a
    string. If any step of this process fails, such as if the file does not exist or
    cannot be read, the function will return an `Err` variant containing the error.
    This pattern of error handling is explicit, making the flow of error
    information clear and predictable.
         */

    match read_file_contents("data.csv") {
        Ok(data) => println!("File contents: {}", data),
        Err(e) => println!("Failed to read file: {}", e),
    }
}
