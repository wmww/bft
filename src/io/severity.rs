use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Severity {
    // Debug,
    // InternalError,
    // Warning,
    // Error,
    RuntimeError,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Severity::Debug => "Debug",
                // Severity::InternalError => "Internal error",
                // Severity::Warning => "Warning",
                // Severity::Error => "Error",
                Severity::RuntimeError => "Runtime error",
            }
        )
    }
}
