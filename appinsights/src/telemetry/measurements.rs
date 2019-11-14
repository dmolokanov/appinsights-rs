use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

/// Contains all measurements for telemetry to submit.
#[derive(Clone, Default)]
pub struct Measurements(BTreeMap<String, f64>);

impl From<Measurements> for BTreeMap<String, f64> {
    fn from(measurements: Measurements) -> Self {
        measurements.0
    }
}

impl Deref for Measurements {
    type Target = BTreeMap<String, f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Measurements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
