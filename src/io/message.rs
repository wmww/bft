use super::*;

pub fn message(severity: Severity, msg: &str) {
    println!(
        "{}: {}",
        match severity {
            Severity::Debug => "Debug",
            //Severity::InternalWarning => "Internal warning",
            Severity::InternalError => "Internal error",
            //Severity::Warning => "Warning",
            Severity::Error => "Error",
            Severity::RuntimeError => "Runtime error",
        },
        msg
    );
}
