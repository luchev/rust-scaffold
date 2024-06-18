use config::ConfigError;
use error_chain::{error_chain, ExitCode};
use runtime_injector::InjectError;
use std::{io, process::exit};
use log::error;

trait ErrorHelper {
    fn help(&self) -> String;
}

pub trait Die {
    fn die(self);
}

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }
    errors {
        UnknownError { display("unknown error") }
        Generic(e: String) { display("{}", e) }
        ConfigErr(e: ConfigError) { display("config error: {}", e) }
        Io(e: io::Error) { display("io error: {}", e) }
        DIError(e: String) { display("di error: {}", e) }
        TonicTransportError(e: tonic::transport::Error) { display("tonic transport error: {}", e) }
    }
}

impl ExitCode for Error {
    fn code(self) -> i32 {
        match self.0 {
            _ => 1,
        }
    }
}

impl ErrorHelper for Error {
    fn help(&self) -> String {
        match self.0 {
            ErrorKind::UnknownError => "No help available for this error",
            _ => "No help available for this error",
        }
        .to_string()
    }
}

impl Die for Error {
    fn die(self) {
        die(self);
    }
}

pub fn die(err: Error) {
    error!("{}", err);
    error!("{}", err.help());
    exit(err.code());
}

impl From<runtime_injector::InjectError> for Error {
    fn from(e: runtime_injector::InjectError) -> Self {
        match e {
            runtime_injector::InjectError::ActivationFailed {
                service_info,
                inner,
            } => ErrorKind::DIError(format!(
                "di: service {} failed: {:?}",
                service_info.name(),
                inner
            ))
            .into(),
            InjectError::CycleDetected { service_info, cycle } => ErrorKind::DIError(format!(
                "cycle detected for service {}: {:?}",
                service_info.name(),
                cycle
            )).into(),
            InjectError::MissingProvider { service_info } => ErrorKind::DIError(format!(
                "missing provider for service {}",
                service_info.name()
            )).into(),
            _ => ErrorKind::DIError("unknown error".to_string()).into(),
        }
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(e: tonic::transport::Error) -> Self {
        ErrorKind::TonicTransportError(e).into()
    }
}
