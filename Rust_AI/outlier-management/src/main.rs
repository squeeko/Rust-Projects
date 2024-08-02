fn z_score(value: f32, mean: f32, std_dev: f32) -> f32 {
    (value - mean) / std_dev
}

fn compute_mean(data: &[f32]) -> f32 {
    data.iter().sum::<f32>() / data.len() as f32
}

fn main() {
    let data_points = vec![10.0, 10.5, 12.4, 2.1, 10.1, 10.3];

    let data_mean = compute_mean(&data_points);

    let data_std_dev = data_points
        .iter()
        .map(|&value| (value - data_mean).powi(2))
        .sum::<f32>()
        .sqrt();

    let data_z_scores: Vec<f32> = data_points
        .iter()
        .map(|&value| z_score(value, data_mean, data_std_dev))
        .collect();

    // Detect outliers wtih a z-score threshold
    let outliers: Vec<f32> = data_points
        .iter()
        .zip(data_z_scores.iter())
        .filter_map(|(&value, &z)| if z.abs() > 2.0 { Some(value) } else { None })
        .collect();

    dbg!(data_mean);
    dbg!(data_std_dev);
    dbg!(data_z_scores);
    dbg!(outliers);
}
