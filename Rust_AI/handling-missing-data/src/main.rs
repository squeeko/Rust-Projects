fn clean_temperature_data(reading: Option<f32>) -> f32 {
    match reading {
        Some(value) => value,
        None => 0.0,
    }
}

fn main() {
    let raw_temperature_data = vec![Some(22.3), None, Some(23.8)];
    let cleaned_data: Vec<f32> = raw_temperature_data
        .into_iter()
        .map(clean_temperature_data)
        .collect();

    println!("Cleaned temperature data: {:?}", cleaned_data);
}
