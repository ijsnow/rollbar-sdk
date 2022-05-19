#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("TransportNotConfigured: must call configure before sending data.")]
    TransportNotConfigured,
    #[error("TransportLock: could not obtain lock on instance.")]
    TransportLock,
    #[error("TransportCreationRaceCondition: tried setting the transport when .")]
    TransportCreationRaceCondition,
    #[error("Transport: {0}")]
    Transport(#[from] crate::transport::Error),
    #[error("Utf8: received a c++ string that was not valid utf8 ({0})")]
    Utf8(#[from] std::str::Utf8Error),
}

#[cxx::bridge]
mod ffi {
    struct Config<'a> {
        uri: &'a CxxString,
        access_token: &'a CxxString,
    }

    extern "Rust" {
        fn configure(config: Config) -> Result<()>;
        fn shutdown() -> Result<()>;

        fn debug(message: &CxxString) -> Result<()>;
        fn info(message: &CxxString) -> Result<()>;
        fn warning(message: &CxxString) -> Result<()>;
        fn error(message: &CxxString) -> Result<()>;
        fn critical(message: &CxxString) -> Result<()>;
    }
}

use ::{
    cxx::CxxString,
    once_cell::sync::OnceCell,
    std::{collections::HashMap, sync::Mutex},
};

use crate::{
    types::{Item, Level},
    Config, Transport,
};

fn config_from_shared_config(config: ffi::Config) -> Result<Config, Error> {
    Ok(Config::builder()
        .access_token(config.access_token.to_str()?.to_owned())
        .uri(config.uri.to_str()?.to_owned())
        .build())
}

static TRANSPORT: OnceCell<Mutex<Transport>> = OnceCell::new();

pub fn configure(config: ffi::Config) -> Result<(), Error> {
    let config = config_from_shared_config(config)?;

    match TRANSPORT.get() {
        Some(instance) => {
            let transport = instance.lock().map_err(|_| Error::TransportLock)?;

            transport.set_config(config)?;
        }
        None => {
            let transport = Transport::new(config)?;

            TRANSPORT
                .set(Mutex::new(transport))
                .map_err(|_| Error::TransportCreationRaceCondition)?;
        }
    };

    Ok(())
}

pub fn shutdown() -> Result<(), Error> {
    let transport = TRANSPORT.get().ok_or(Error::TransportNotConfigured)?;

    let transport = transport.lock().map_err(|_| Error::TransportLock)?;

    transport.shutdown().map_err(Error::from)
}

fn log(level: Level, message: &CxxString) -> Result<(), Error> {
    let transport = TRANSPORT.get().ok_or(Error::TransportNotConfigured)?;

    let transport = transport.lock().map_err(|_| Error::TransportLock)?;

    let item = Item::from((level, message.to_str()?, HashMap::new()));

    transport.send(item)?;

    Ok(())
}

pub fn debug(message: &CxxString) -> Result<(), Error> {
    log(Level::Debug, message)
}

pub fn info(message: &CxxString) -> Result<(), Error> {
    log(Level::Info, message)
}

pub fn warning(message: &CxxString) -> Result<(), Error> {
    log(Level::Warning, message)
}

pub fn error(message: &CxxString) -> Result<(), Error> {
    log(Level::Error, message)
}

pub fn critical(message: &CxxString) -> Result<(), Error> {
    log(Level::Critical, message)
}
