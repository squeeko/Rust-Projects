// Data Type Conversions and Strategies for Missing Data
use std::collections::HashMap;

fn compute_mean(data: &[f32]) -> f32 {
    data.iter().sum::<f32>() / data.len() as f32
}

fn impute_missing_values(data: &mut HashMap<usize, Option<f32>>) {
    let existing_vals: Vec<f32> = data.values().filter_map(|&x| x).collect();
    let mean_val = compute_mean(&existing_vals);

    data.iter_mut().for_each(|(_, v)| {
        if v.is_none() {
            *v = Some(mean_val);
        }
    });
}

fn main() {
    // Data Type Conversions
    let string_data = vec!["3.14", "not-a-number", "2.71"];
    let numerical_data: Vec<f32> = string_data
        .into_iter()
        .filter_map(|s| s.parse::<f32>().ok())
        .collect();

    println!("Converting strings to f32: {:?}", numerical_data);

    // Strategies for Missing Data

    let find_mean: f32 = compute_mean(&[23.0, 90.5, 56.0, 9.0, 43.8]);
    println!("Finding the mean: {:?}", find_mean);

    let mut temperature_data: HashMap<usize, Option<f32>> =
        HashMap::from([(0, Some(20.5)), (1, None), (2, Some(22.1)), (3, None)]);

    impute_missing_values(&mut temperature_data);
}
