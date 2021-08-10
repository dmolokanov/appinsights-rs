/// Stores statistics for aggregated metric.
#[derive(Debug, PartialEq, Default)]
pub struct Stats {
    /// Sampled value.
    pub(crate) value: f64,

    /// Minimum value of the aggregated metric.
    pub(crate) min: f64,

    /// Maximum value of the aggregated metric.
    pub(crate) max: f64,

    /// Count of measurements in the sample.
    pub(crate) count: i32,

    /// Standard deviation of the aggregated metric.
    pub(crate) std_dev: f64,
}

impl Stats {
    /// Adds data points to the aggregate totals included in this telemetry item.
    /// This can be used for all the data at once or incrementally. Calculates
    /// min, max, sum, count, and std_dev (by way of variance).
    pub fn add_data(&mut self, values: &[f64]) {
        let variance_sum = if self.std_dev != 0.0 {
            self.std_dev * self.std_dev * self.count as f64
        } else {
            0.0
        };

        let variance_sum = self.add_values(values, variance_sum);
        if self.count > 0 {
            let variance = variance_sum / self.count as f64;
            self.std_dev = f64::sqrt(variance);
        }
    }

    /// Adds sampled data points to the aggregate totals included in this telemetry item.
    /// This can be used for all the data at once or incrementally. Differs from [add_data](#method.add_data)
    /// in how it calculates standard deviation, and should not be used interchangeably
    /// with [add_data](#method.add_data)
    pub fn add_sampled_data(&mut self, values: &[f64]) {
        let variance_sum = if self.std_dev != 0.0 {
            self.std_dev * self.std_dev * self.count as f64
        } else {
            0.0
        };

        let variance_sum = self.add_values(values, variance_sum);
        if self.count > 1 {
            let variance = variance_sum / (self.count - 1) as f64;
            self.std_dev = f64::sqrt(variance);
        }
    }

    fn add_values(&mut self, values: &[f64], variance_sum: f64) -> f64 {
        let mut variance_sum = variance_sum;
        if !values.is_empty() {
            // running tally of the mean is important for incremental variance computation
            let mut mean = 0.0;
            if self.count == 0 {
                self.min = values[0];
                self.max = values[0];
            } else {
                mean = self.value / self.count as f64;
            }

            self.min = values.iter().fold(std::f64::NAN, |x, min| min.min(x));
            self.max = values.iter().fold(std::f64::NAN, |x, max| max.max(x));

            // Welford's algorithm to compute variance. The divide occurs in the caller.
            let mut value = self.value;
            let mut count = self.count;
            for x in values {
                count += 1;
                value += *x;
                let new_mean = value / count as f64;
                variance_sum += (x - mean) * (x - new_mean);
                mean = new_mean;
            }
            self.count = count;
            self.value = value;
        }

        variance_sum
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(&[],                           0.0,    0.0,    0.0     ; "for empty collection")]
    #[test_case(&[0.0],                        0.0,    0.0,    0.0     ; "for single zero value")]
    #[test_case(&[50.0],                       0.0,    50.0,   50.0    ; "for single non-zero value")]
    #[test_case(&[50.0, 50.0],                 0.0,    50.0,   50.0    ; "for two equal non-zero values")]
    #[test_case(&[50.0, 60.0],                 5.0,    50.0,   60.0    ; "for two non-equal non-zero values")]
    #[test_case(&[9.0, 10.0, 11.0, 7.0, 13.0], 2.0,    7.0,    13.0    ; "for several values")]
    fn it_calculates_stats(values: &[f64], std_dev: f64, min: f64, max: f64) {
        let mut stats = Stats::default();
        stats.add_data(values);

        assert_eq!(
            stats,
            Stats {
                value: values.iter().sum(),
                min,
                max,
                count: values.len() as i32,
                std_dev,
            }
        )
    }

    #[test_case(&[],                           0.0,    0.0,    0.0     ; "for empty collection")]
    #[test_case(&[0.0],                        0.0,    0.0,    0.0     ; "for single zero value")]
    #[test_case(&[50.0],                       0.0,    50.0,   50.0    ; "for single non-zero value")]
    #[test_case(&[50.0, 50.0],                 0.0,    50.0,   50.0    ; "for two equal non-zero values")]
    #[test_case(&[50.0, 60.0],                 7.0710678118654755,    50.0,   60.0    ; "for two non-equal non-zero values")]
    #[test_case(&[9.0, 10.0, 11.0, 7.0, 13.0], 2.23606797749979,    7.0,    13.0    ; "for several values")]
    fn it_calculates_sampled_stats(values: &[f64], std_dev: f64, min: f64, max: f64) {
        let mut stats = Stats::default();
        stats.add_sampled_data(values);

        assert_eq!(
            stats,
            Stats {
                value: values.iter().sum(),
                min,
                max,
                count: values.len() as i32,
                std_dev,
            }
        )
    }
}
