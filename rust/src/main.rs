mod classical;
mod data;
mod tensor_net;

use ndarray::Array1;
use plotters::prelude::*;
use std::path::Path;

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
    println!("  Plot saved: {filename}");
    Ok(())
}

fn plot_comparison(
    tensor_trend: &Array1<f64>,
    classical_trend: &Array1<f64>,
    label_a: &str,
    label_b: &str,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (1200, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let n = tensor_trend.len() as f64;
    let all_vals: Vec<f64> = tensor_trend
        .iter()
        .chain(classical_trend.iter())
        .cloned()
        .collect();
    let min_val = all_vals.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = all_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let margin = (max_val - min_val) * 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Trend Comparison", ("sans-serif", 22).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..n, (min_val - margin)..(max_val + margin))?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            tensor_trend.iter().enumerate().map(|(x, y)| (x as f64, *y)),
            &RED,
        ))?
        .label(label_a)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(LineSeries::new(
            classical_trend
                .iter()
                .enumerate()
                .map(|(x, y)| (x as f64, *y)),
            GREEN.stroke_width(2),
        ))?
        .label(label_b)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    println!("  Plot saved: {filename}");
    Ok(())
}

fn rmse(a: &Array1<f64>, b: &Array1<f64>) -> f64 {
    let diff = a - b;
    (diff.mapv(|x| x * x).mean().unwrap_or(0.0)).sqrt()
}

fn correlation(a: &Array1<f64>, b: &Array1<f64>) -> f64 {
    let _n = a.len() as f64;
    let mean_a = a.mean().unwrap_or(0.0);
    let mean_b = b.mean().unwrap_or(0.0);
    let mut cov = 0.0;
    let mut var_a = 0.0;
    let mut var_b = 0.0;
    for i in 0..a.len() {
        let da = a[i] - mean_a;
        let db = b[i] - mean_b;
        cov += da * db;
        var_a += da * da;
        var_b += db * db;
    }
    if var_a < 1e-15 || var_b < 1e-15 {
        return 0.0;
    }
    cov / (var_a.sqrt() * var_b.sqrt())
}

fn run_synthetic() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SYNTHETIC DATA (365 days) ===\n");

    let (series, ground_truth) = data::generate_time_series(365);
    println!(
        "Generated: {} points (trend + weekly seasonality + noise)",
        series.len()
    );

    // Spectrum
    let spectrum = tensor_net::singular_value_spectrum(&series, 7);
    println!("\nSingular value energy spectrum (window=7):");
    for (i, pct) in spectrum.iter().enumerate() {
        let bar = "█".repeat((pct / 2.0) as usize);
        println!("  σ{i}: {pct:6.2}% {bar}");
    }

    // Tensor decomposition
    let tensor = tensor_net::decompose_tensor(&series, 7, 1, 2);
    println!("\n--- Tensor Network (SSA) ---");
    println!(
        "  Trend RMSE:       {:.4}",
        rmse(&tensor.trend, &ground_truth.trend)
    );
    println!(
        "  Seasonality RMSE: {:.4}",
        rmse(&tensor.seasonality, &ground_truth.seasonality)
    );
    println!(
        "  Noise std:        {:.4} (expected ≈0.29)",
        tensor.noise.std(0.0)
    );

    // Classical decomposition
    let cl_trend = classical::moving_average(&series, 3);
    let cl_season = classical::extract_seasonality(&series, &cl_trend, 7);
    let cl_noise = &series - &cl_trend - &cl_season;
    println!("\n--- Classical (7-point MA) ---");
    println!(
        "  Trend RMSE:       {:.4}",
        rmse(&cl_trend, &ground_truth.trend)
    );
    println!(
        "  Seasonality RMSE: {:.4}",
        rmse(&cl_season, &ground_truth.seasonality)
    );
    println!(
        "  Noise std:        {:.4} (expected ≈0.29)",
        cl_noise.std(0.0)
    );

    // Plots
    plot_decomposition(
        &series,
        &tensor.trend,
        &tensor.seasonality,
        &tensor.noise,
        "Tensor Network (SSA) — Synthetic",
        "plots/synthetic_tensor.png",
    )?;
    plot_decomposition(
        &series,
        &cl_trend,
        &cl_season,
        &cl_noise,
        "Classical (MA) — Synthetic",
        "plots/synthetic_classical.png",
    )?;

    Ok(())
}

fn run_stock(csv_path: &str, name: &str, period: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "\n=== STOCK DATA: {} ({} trading days/period) ===\n",
        name, period
    );

    let (_dates, series) = data::load_csv(Path::new(csv_path))?;
    let n = series.len();
    let min_price = series.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_price = series.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let returns: Vec<f64> = (1..n)
        .map(|i| (series[i] / series[i - 1] - 1.0) * 100.0)
        .collect();
    let avg_return = returns.iter().sum::<f64>() / returns.len() as f64;
    let volatility = (returns
        .iter()
        .map(|r| (r - avg_return).powi(2))
        .sum::<f64>()
        / returns.len() as f64)
        .sqrt();

    println!("  Points: {n}");
    println!("  Range:  ${min_price:.2} — ${max_price:.2}");
    println!(
        "  Total return: {:.1}%",
        (series[n - 1] / series[0] - 1.0) * 100.0
    );
    println!("  Avg daily return: {avg_return:.3}%");
    println!("  Daily volatility: {volatility:.3}%");
    println!(
        "  Annualized vol:   {:.1}%",
        volatility * (252.0_f64).sqrt()
    );

    // Spectrum analysis — try weekly (5 trading days) window
    let spectrum = tensor_net::singular_value_spectrum(&series, period);
    println!("\n  Singular value energy spectrum (window={period}):");
    for (i, pct) in spectrum.iter().take(10).enumerate() {
        let bar = "█".repeat((pct * 0.5) as usize);
        println!("    σ{i}: {pct:6.2}% {bar}");
    }

    // Tensor decomposition
    // For stock prices: rank 1-2 for trend, rank 2-4 for weekly patterns
    let tensor = tensor_net::decompose_tensor(&series, period, 2, 3);
    println!("\n  --- Tensor Network (SSA, window={period}) ---");
    println!(
        "    Trend range: ${:.2} — ${:.2}",
        tensor.trend.iter().cloned().fold(f64::INFINITY, f64::min),
        tensor
            .trend
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max)
    );
    println!(
        "    Seasonality amplitude: ±${:.2}",
        tensor
            .seasonality
            .iter()
            .map(|x| x.abs())
            .fold(0.0_f64, f64::max)
    );
    println!("    Noise std: ${:.2}", tensor.noise.std(0.0));
    println!(
        "    Trend-original correlation: {:.4}",
        correlation(&tensor.trend, &series)
    );

    // Classical decomposition
    let half_window = period / 2;
    let cl_trend = classical::moving_average(&series, half_window.max(1));
    let cl_season = classical::extract_seasonality(&series, &cl_trend, period);
    let cl_noise = &series - &cl_trend - &cl_season;
    println!("\n  --- Classical ({}-point MA) ---", 2 * half_window + 1);
    println!(
        "    Trend-original correlation: {:.4}",
        correlation(&cl_trend, &series)
    );
    println!(
        "    Seasonality amplitude: ±${:.2}",
        cl_season.iter().map(|x| x.abs()).fold(0.0_f64, f64::max)
    );
    println!("    Noise std: ${:.2}", cl_noise.std(0.0));

    // Comparison
    println!("\n  --- Comparison ---");
    println!(
        "    Trend correlation:  tensor={:.4}  classical={:.4}",
        correlation(&tensor.trend, &series),
        correlation(&cl_trend, &series)
    );
    println!(
        "    Noise std:          tensor=${:.2}  classical=${:.2}",
        tensor.noise.std(0.0),
        cl_noise.std(0.0)
    );

    // Plots
    let prefix = name.to_lowercase().replace(' ', "_");
    plot_decomposition(
        &series,
        &tensor.trend,
        &tensor.seasonality,
        &tensor.noise,
        &format!("Tensor Network — {name}"),
        &format!("plots/{prefix}_tensor.png"),
    )?;
    plot_decomposition(
        &series,
        &cl_trend,
        &cl_season,
        &cl_noise,
        &format!("Classical (MA) — {name}"),
        &format!("plots/{prefix}_classical.png"),
    )?;
    plot_comparison(
        &tensor.trend,
        &cl_trend,
        "Tensor (SSA)",
        "Classical (MA)",
        &format!("plots/{prefix}_trend_compare.png"),
    )?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  Quantum-Inspired Time Series Decomposition         ║");
    println!("║  Tensor Networks (SSA) vs Classical Methods         ║");
    println!("╚══════════════════════════════════════════════════════╝\n");

    // 1. Synthetic benchmark
    run_synthetic()?;

    // 2. Real stock data
    let stock_csv = "data/aapl_1y.csv";
    if Path::new(stock_csv).exists() {
        // Weekly period = 5 trading days
        run_stock(stock_csv, "AAPL (simulated)", 5)?;

        // Also try monthly period = 21 trading days
        run_stock(stock_csv, "AAPL monthly", 21)?;
    } else {
        println!("\n⚠️  No stock data found at {stock_csv}");
        println!("    Generate with: python3 scripts/fetch_stock.py AAPL");
    }

    println!("\n✅ Done! Check plots/ for visualizations.");
    Ok(())
}
