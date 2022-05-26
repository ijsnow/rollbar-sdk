use crate::{
    types::{Item, Level},
    Config, Transport,
};

use ::{
    libc::{c_char, c_int},
    once_cell::sync::OnceCell,
    std::{collections::HashMap, ffi::CStr},
};

#[repr(C)]
pub struct ConfigCompat {
    uri: *const c_char,
    access_token: *const c_char,
}

#[repr(C)]
pub enum LevelCompat {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl From<LevelCompat> for Level {
    fn from(level: LevelCompat) -> Level {
        use LevelCompat::*;

        match level {
            Debug => Level::Debug,
            Info => Level::Info,
            Warning => Level::Warning,
            Error => Level::Error,
            Critical => Level::Critical,
        }
    }
}

#[no_mangle]
pub extern "C" fn create_transport(
    in_config: ConfigCompat,
    out_transport: &mut *mut Transport,
) -> c_int {
    let access_token = match unsafe { CStr::from_ptr(in_config.access_token) }.to_str() {
        Ok(access_token) => access_token.to_owned(),
        Err(error) => {
            eprintln!("access_token required: {}", error);
            return 0;
        }
    };

    let uri = match unsafe { CStr::from_ptr(in_config.uri) }.to_str() {
        Ok(uri) => uri.to_owned(),
        Err(_) => Config::default_uri(),
    };

    let config = Config { access_token, uri };

    match Transport::new(config) {
        Ok(transport) => {
            *out_transport = Box::into_raw(Box::new(transport));
            0
        }
        Err(error) => {
            eprintln!("error creating transport: {}", error);
            1
        }
    }
}

#[no_mangle]
pub extern "C" fn log(
    transport: &mut Transport,
    level: LevelCompat,
    message: *const c_char,
) -> c_int {
    let message = match unsafe { CStr::from_ptr(message) }.to_str() {
        Ok(message) => message,
        Err(error) => {
            eprintln!("error casting string: {}", error);
            return 1;
        }
    };

    let item = Item::from((Level::from(level), message, HashMap::new()));

    match transport.send(item) {
        Err(error) => {
            eprintln!("error sending item: {}", error);
            1
        }
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn shutdown(transport: &mut Transport) -> c_int {
    match transport.shutdown() {
        Err(error) => {
            eprintln!("error sending item: {}", error);
            1
        }
        _ => 0,
    }
}
