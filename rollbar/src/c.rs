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
    pub uri: *const c_char,
    pub access_token: *const c_char,
}

#[no_mangle]
pub extern "C" fn create_transport(
    in_config: ConfigCompat,
    out_transport: &mut *mut Transport,
) -> c_int {
    let access_token = {
        if in_config.access_token.is_null() {
            eprintln!("access_token required");
            return 0;
        }

        match unsafe { CStr::from_ptr(in_config.access_token) }.to_str() {
            Ok(access_token) => access_token.to_owned(),
            Err(error) => {
                eprintln!("access_token not utf8: {}", error);
                return 0;
            }
        }
    };

    let uri = {
        if in_config.uri.is_null() {
            Config::default_uri()
        } else {
            match unsafe { CStr::from_ptr(in_config.uri) }.to_str() {
                Ok(uri) => uri.to_owned(),
                Err(error) => {
                    eprintln!("uri not utf8: {}", error);
                    return 0;
                }
            }
        }
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
pub extern "C" fn log(transport: &mut Transport, level: Level, message: *const c_char) -> c_int {
    let message = match unsafe { CStr::from_ptr(message) }.to_str() {
        Ok(message) => message,
        Err(error) => {
            eprintln!("error casting string: {}", error);
            return 1;
        }
    };

    let item = Item::from((level, message, HashMap::new()));

    transport.send(item);

    0
}
