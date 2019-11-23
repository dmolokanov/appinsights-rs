/// Describes command to be sent to internal channel.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// A command to tear down the submission, close internal channels. All pending telemetry items to be discarded.
    Terminate,

    /// A command to force all pending telemetry items to be submitted.
    Flush,

    /// A command to tear down the submission, close internal channels and wait until all pending telemetry items to be sent.
    Close,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Command::Flush => "flush",
            Command::Terminate => "terminate",
            Command::Close => "close",
        };
        write!(f, "{}", label)
    }
}
