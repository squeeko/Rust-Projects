fn normalize(value: f32, min: f32, max: f32) -> f32 {
    (value - min) / (max - min)
}

fn main() {
    let sensor_data = vec![12.0, 75.2, 56.4, 84.3];

    let min_value = *sensor_data
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let max_value = *sensor_data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let normalized_data: Vec<f32> = sensor_data
        .into_iter()
        .map(|value| normalize(value, min_value, max_value))
        .collect();

    println!("Normalized Data: {:?}", normalized_data);
}
