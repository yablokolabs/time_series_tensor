mod classical;
mod data;
mod tensor_net;

use ndarray::Array1;
use plotters::prelude::*;

fn plot_decomposition(
    series: &Array1<f64>,
    trend: &Array1<f64>,
    seasonality: &Array1<f64>,
    noise: &Array1<f64>,
    title: &str,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (1200, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let areas = root.split_evenly((4, 1));
    let n = series.len() as f64;

    let components: Vec<(&str, &Array1<f64>, &RGBColor)> = vec![
        ("Original", series, &BLUE),
        ("Trend", trend, &RED),
        ("Seasonality", seasonality, &GREEN),
        ("Noise", noise, &BLACK),
    ];

    for (area, (label, data, color)) in areas.iter().zip(components.iter()) {
        let min_val = data.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let margin = (max_val - min_val).max(0.1) * 0.15;

        let mut chart = ChartBuilder::on(area)
            .caption(format!("{title} — {label}"), ("sans-serif", 18).into_font())
            .margin(5)
            .x_label_area_size(25)
            .y_label_area_size(50)
            .build_cartesian_2d(0.0..n, (min_val - margin)..(max_val + margin))?;

        chart.configure_mesh().light_line_style(WHITE).draw()?;

        chart.draw_series(LineSeries::new(
            data.iter().enumerate().map(|(x, y)| (x as f64, *y)),
            color.stroke_width(1),
        ))?;
    }

    root.present()?;
    println!("Plot saved: {filename}");
    Ok(())
}

fn plot_comparison(
    tensor_trend: &Array1<f64>,
    classical_trend: &Array1<f64>,
    ground_truth_trend: &Array1<f64>,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (1200, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let n = tensor_trend.len() as f64;
    let all_vals: Vec<f64> = tensor_trend
        .iter()
        .chain(classical_trend.iter())
        .chain(ground_truth_trend.iter())
        .cloned()
        .collect();
    let min_val = all_vals.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = all_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let margin = (max_val - min_val) * 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Trend Comparison: Tensor vs Classical vs Ground Truth",
            ("sans-serif", 22).into_font(),
        )
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..n, (min_val - margin)..(max_val + margin))?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            ground_truth_trend
                .iter()
                .enumerate()
                .map(|(x, y)| (x as f64, *y)),
            &BLUE,
        ))?
        .label("Ground Truth")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .draw_series(LineSeries::new(
            tensor_trend.iter().enumerate().map(|(x, y)| (x as f64, *y)),
            &RED,
        ))?
        .label("Tensor Network")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(LineSeries::new(
            classical_trend
                .iter()
                .enumerate()
                .map(|(x, y)| (x as f64, *y)),
            GREEN.stroke_width(2),
        ))?
        .label("Classical MA")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    println!("Plot saved: {filename}");
    Ok(())
}

fn rmse(a: &Array1<f64>, b: &Array1<f64>) -> f64 {
    let diff = a - b;
    (diff.mapv(|x| x * x).mean().unwrap_or(0.0)).sqrt()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Quantum-Inspired Time Series Decomposition ===\n");

    // 1. Generate synthetic data
    let (series, ground_truth) = data::generate_time_series(365);
    println!("Generated time series: {} points", series.len());
    println!(
        "  First 10: {:?}",
        &series.as_slice().unwrap()[..10]
            .iter()
            .map(|x| format!("{x:.2}"))
            .collect::<Vec<_>>()
    );

    // 2. Singular value spectrum analysis
    let spectrum = tensor_net::singular_value_spectrum(&series, 7);
    println!("\nSingular value energy spectrum (window=7):");
    for (i, pct) in spectrum.iter().enumerate() {
        let bar = "█".repeat((pct / 2.0) as usize);
        println!("  σ{i}: {pct:6.2}% {bar}");
    }

    // 3. Tensor network decomposition
    // trend_rank=1: captures the linear trend (dominant singular value)
    // seasonal_rank=2: captures sin+cos pair for weekly seasonality
    let tensor_result = tensor_net::decompose_tensor(&series, 7, 1, 2);
    println!("\n--- Tensor Network Decomposition ---");
    println!(
        "  Trend RMSE vs ground truth:       {:.4}",
        rmse(&tensor_result.trend, &ground_truth.trend)
    );
    println!(
        "  Seasonality RMSE vs ground truth:  {:.4}",
        rmse(&tensor_result.seasonality, &ground_truth.seasonality)
    );
    println!(
        "  Noise std (should be ≈0.29):       {:.4}",
        tensor_result.noise.std(0.0)
    );

    // 4. Classical decomposition (baseline)
    let classical_trend = classical::moving_average(&series, 3); // half_window=3 → 7-point MA
    let classical_seasonality = classical::extract_seasonality(&series, &classical_trend, 7);
    let classical_noise = &series - &classical_trend - &classical_seasonality;
    println!("\n--- Classical Decomposition (7-point MA) ---");
    println!(
        "  Trend RMSE vs ground truth:       {:.4}",
        rmse(&classical_trend, &ground_truth.trend)
    );
    println!(
        "  Seasonality RMSE vs ground truth:  {:.4}",
        rmse(&classical_seasonality, &ground_truth.seasonality)
    );
    println!(
        "  Noise std (should be ≈0.29):       {:.4}",
        classical_noise.std(0.0)
    );

    // 5. Visualize
    plot_decomposition(
        &series,
        &tensor_result.trend,
        &tensor_result.seasonality,
        &tensor_result.noise,
        "Tensor Network (SSA)",
        "plots/tensor_decomposition.png",
    )?;

    plot_decomposition(
        &series,
        &classical_trend,
        &classical_seasonality,
        &classical_noise,
        "Classical (Moving Average)",
        "plots/classical_decomposition.png",
    )?;

    plot_comparison(
        &tensor_result.trend,
        &classical_trend,
        &ground_truth.trend,
        "plots/trend_comparison.png",
    )?;

    println!("\n✅ Done! Check plots/ for visualizations.");

    Ok(())
}
