// NOTE: This file was automatically generated.

#![allow(unused_variables, dead_code, unused_imports)]

mod availabilitydata;
mod base;
mod contexttagkeys;
mod data;
mod datapoint;
mod datapointtype;
mod domain;
mod envelope;
mod eventdata;
mod exceptiondata;
mod exceptiondetails;
mod messagedata;
mod metricdata;
mod pageviewdata;
mod remotedependencydata;
mod requestdata;
mod severitylevel;
mod stackframe;

pub use availabilitydata::*;
pub use base::*;
pub use contexttagkeys::*;
pub use data::*;
pub use datapoint::*;
pub use datapointtype::*;
pub use domain::*;
pub use envelope::*;
pub use eventdata::*;
pub use exceptiondata::*;
pub use exceptiondetails::*;
pub use messagedata::*;
pub use metricdata::*;
pub use pageviewdata::*;
pub use remotedependencydata::*;
pub use requestdata::*;
pub use severitylevel::*;
pub use stackframe::*;

/// Common interface implemented by telemetry data contacts.
pub trait TelemetryData {
    /// Returns the name used when this is embedded within an [Envelope](trait.Envelope.html) container.
    fn envelope_name(&self, key: &str) -> String {
        let mut name = self.base_type();
        name.truncate(name.len() - 4);

        if key.is_empty() {
            format!("Microsoft.ApplicationInsights.{}.{}", key, name)
        } else {
            format!("Microsoft.ApplicationInsights.{}", name)
        }
    }

    /// Returns the base type when placed within an [Data](trait.Data.html) container.
    fn base_type(&self) -> String;
}
