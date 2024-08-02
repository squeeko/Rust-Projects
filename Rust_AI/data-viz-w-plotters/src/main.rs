use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use plotters for data viz

    let root = BitMapBackend::new("line_chart.png", (640, 480)).into_drawing_area(); // created the .png file
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Example Line Chart", ("sans-serif", 40))
        .margin(5)
        .x_label_area_size(15)
        .y_label_area_size(40)
        .build_cartesian_2d(0..10, 0..10)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new((0..10).map(|x| (x, x)), &RED))?;

    root.present()?;
    Ok(())
}
