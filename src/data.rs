use ndarray::Array1;
use rand::Rng;
use std::fs;
use std::path::Path;

/// Ground-truth components for validation.
pub struct GroundTruth {
    pub trend: Array1<f64>,
    pub seasonality: Array1<f64>,
    pub noise: Array1<f64>,
}

/// Load a time series from a CSV file with columns: date/timestamp, close.
/// Returns the close prices as an Array1.
pub fn load_csv(path: &Path) -> Result<(Vec<String>, Array1<f64>), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let mut dates = Vec::new();
    let mut prices = Vec::new();

    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            continue; // skip header
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 2 {
            dates.push(parts[0].to_string());
            prices.push(parts[1].parse::<f64>()?);
        }
    }

    let array = Array1::from_vec(prices);
    Ok((dates, array))
}

/// Generate a synthetic time series: y = trend + seasonality + noise.
///
/// - Trend: linear `0.1 * t`
/// - Seasonality: weekly `sin(2πt/7)`
/// - Noise: Gaussian `N(0, 0.5)`
pub fn generate_time_series(length: usize) -> (Array1<f64>, GroundTruth) {
    let mut rng = rand::thread_rng();
    let mut series = Array1::<f64>::zeros(length);
    let mut trend = Array1::<f64>::zeros(length);
    let mut seasonality = Array1::<f64>::zeros(length);
    let mut noise = Array1::<f64>::zeros(length);

    for t in 0..length {
        let tr = 0.1 * (t as f64);
        let se = (2.0 * std::f64::consts::PI * (t as f64) / 7.0).sin();
        let no: f64 = rng.gen::<f64>() * 1.0 - 0.5; // Uniform [-0.5, 0.5]

        trend[t] = tr;
        seasonality[t] = se;
        noise[t] = no;
        series[t] = tr + se + no;
    }

    (
        series,
        GroundTruth {
            trend,
            seasonality,
            noise,
        },
    )
}
