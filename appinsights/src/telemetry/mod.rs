mod availability;
mod event;
mod measurements;
mod metric;
mod page_view;
mod properties;
mod remote_dependency;
mod request;
mod tags;
mod trace;

pub use availability::*;
pub use event::*;
pub use measurements::*;
pub use metric::*;
pub use page_view::*;
pub use properties::*;
pub use remote_dependency::*;
pub use request::*;
pub use tags::*;
pub use trace::*;

use chrono::{DateTime, Utc};

/// A trait that provides Application Insights telemetry items.
pub trait Telemetry {
    /// Returns the time when this telemetry was measured.
    fn timestamp(&self) -> DateTime<Utc>;

    /// Returns custom properties to submit with the telemetry item.
    fn properties(&self) -> &Properties;

    /// Returns mutable reference to custom properties.
    fn properties_mut(&mut self) -> &mut Properties;

    /// Returns context data containing extra, optional tags. Overrides values found on client telemetry context.
    fn tags(&self) -> &ContextTags;

    /// Returns mutable reference to custom tags.
    fn tags_mut(&mut self) -> &mut ContextTags;
}
