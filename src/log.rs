// use source;

pub enum Severity {
    Debug,
    //InternalWarning,
    InternalError,
    //Warning,
    //Error,
}

pub fn message(severity: Severity, msg: &str) {
    println!(
        "{}: {}",
        match severity {
            Severity::Debug => "Debug",
            //Severity::InternalWarning => "Internal warning",
            Severity::InternalError => "Internal error",
            //Severity::Warning => "Warning",
            //Severity::Error => "Error",
        },
        msg
    );
}

/*
pub struct Issue {
    severity: Severity,
    span: Option<source::Span>,
    message: String,
}

impl Issue {
    fn show(&self) {
        message(self.severity, format!("{}:\n    {}", self.span,  self.message));
    }
}
*/

#[macro_export]
macro_rules! bft_error {
    ($ops:ident, $($arg:tt)*) => ({
        log::message(log::Severity::InternalError, &format!($($arg)*));
        std::process::exit(1);
    })
}

#[macro_export]
macro_rules! bft_log {
    ($ops:ident, $($arg:tt)*) => ({
        if $ops.debug {
            log::message(log::Severity::Debug, &format!($($arg)*));
        }
    })
}
