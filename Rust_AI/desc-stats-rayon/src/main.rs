use incr_stats::batch;
use rayon::prelude::*;

fn main() {
    let data = vec![2.3, 3.7, 4.1, 5.0, 6.2];
    println!("The data mean: {:?}", batch::mean(&data));
    println!(
        "The data std dev: {:?}",
        batch::sample_standard_deviation(&data)
    );

    let large_data: Vec<f64> = (0..1000000).map(|x| x as f64).collect();
    let mean_value: f64 = large_data.par_iter().sum::<f64>() / large_data.len() as f64;
    println!("The large data mean: {:?}", mean_value);
}
