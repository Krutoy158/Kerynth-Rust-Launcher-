use thiserror::Error;

pub const MIN_RELIABLE_PERCENTILE_SAMPLES: usize = 1_000;

#[derive(Clone, Debug, PartialEq)]
pub struct FrameMetrics {
    pub samples: usize,
    pub average_fps: f64,
    pub one_percent_low_fps: f64,
    pub median_frame_ms: f64,
    pub p95_frame_ms: f64,
    pub p99_frame_ms: f64,
    pub reliable_percentiles: bool,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum BenchmarkError {
    #[error("at least one frame sample is required")]
    EmptySamples,
    #[error("frame times must be finite positive numbers")]
    InvalidFrameTime,
}

impl FrameMetrics {
    pub fn from_frame_times_ms(samples: &[f64]) -> Result<Self, BenchmarkError> {
        if samples.is_empty() {
            return Err(BenchmarkError::EmptySamples);
        }

        if samples
            .iter()
            .any(|value| !value.is_finite() || *value <= 0.0)
        {
            return Err(BenchmarkError::InvalidFrameTime);
        }

        let mut sorted = samples.to_vec();
        sorted.sort_by(f64::total_cmp);

        let average_frame_ms = sorted.iter().sum::<f64>() / sorted.len() as f64;
        let slow_count = (sorted.len() as f64 * 0.01).ceil() as usize;
        let slow_start = sorted.len().saturating_sub(slow_count);
        let slow_average_ms =
            sorted[slow_start..].iter().sum::<f64>() / slow_count as f64;

        Ok(Self {
            samples: sorted.len(),
            average_fps: 1_000.0 / average_frame_ms,
            one_percent_low_fps: 1_000.0 / slow_average_ms,
            median_frame_ms: percentile(&sorted, 0.50),
            p95_frame_ms: percentile(&sorted, 0.95),
            p99_frame_ms: percentile(&sorted, 0.99),
            reliable_percentiles: sorted.len() >= MIN_RELIABLE_PERCENTILE_SAMPLES,
        })
    }
}

fn percentile(sorted: &[f64], percentile: f64) -> f64 {
    let index = ((sorted.len() as f64 * percentile).ceil() as usize)
        .saturating_sub(1)
        .min(sorted.len() - 1);
    sorted[index]
}

#[cfg(test)]
mod tests {
    use super::{BenchmarkError, FrameMetrics};

    #[test]
    fn calculates_average_and_low_fps() {
        let metrics = FrameMetrics::from_frame_times_ms(&[
            10.0, 10.0, 10.0, 20.0, 100.0,
        ])
        .expect("samples should be valid");

        assert!((metrics.average_fps - 33.333333).abs() < 0.001);
        assert!((metrics.one_percent_low_fps - 10.0).abs() < 0.001);
        assert!(!metrics.reliable_percentiles);
    }

    #[test]
    fn rejects_empty_samples() {
        assert_eq!(
            FrameMetrics::from_frame_times_ms(&[]).unwrap_err(),
            BenchmarkError::EmptySamples
        );
    }

    #[test]
    fn rejects_non_positive_frame_time() {
        assert_eq!(
            FrameMetrics::from_frame_times_ms(&[16.0, 0.0]).unwrap_err(),
            BenchmarkError::InvalidFrameTime
        );
    }
}
