use ndarray::Array1;

/// Classical centered moving average for trend extraction.
///
/// Uses a symmetric window of `2*half_window + 1` points.
/// Boundary values are set to the nearest valid average.
pub fn moving_average(series: &Array1<f64>, half_window: usize) -> Array1<f64> {
    let n = series.len();
    let mut trend = Array1::<f64>::zeros(n);
    let _full_window = 2 * half_window + 1;

    for i in 0..n {
        let start = i.saturating_sub(half_window);
        let end = if i + half_window < n {
            i + half_window + 1
        } else {
            n
        };
        let sum: f64 = series.slice(ndarray::s![start..end]).sum();
        let count = end - start;
        trend[i] = sum / count as f64;
    }

    trend
}

/// Extract seasonality by averaging residuals at each period position.
///
/// After removing trend, averages all values at the same position
/// within each period to get the seasonal component.
pub fn extract_seasonality(
    series: &Array1<f64>,
    trend: &Array1<f64>,
    period: usize,
) -> Array1<f64> {
    let n = series.len();
    let detrended = series - trend;
    let mut seasonal = Array1::<f64>::zeros(n);

    // Average by period position
    for pos in 0..period {
        let mut sum = 0.0;
        let mut count = 0;
        for i in (pos..n).step_by(period) {
            sum += detrended[i];
            count += 1;
        }
        let avg = sum / count as f64;
        for i in (pos..n).step_by(period) {
            seasonal[i] = avg;
        }
    }

    seasonal
}
