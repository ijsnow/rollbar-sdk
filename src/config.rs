use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Config {
    access_token: String,
    #[serde(default = "Config::default_endpoint")]
    endpoint: String,
    #[serde(default = "Config::default_shutdown_timeout")]
    shutdown_timeout: u64,
}

impl Config {
    pub fn default_endpoint() -> String {
        "https://api.rollbar.com/api/1/item".into()
    }

    pub fn default_shutdown_timeout() -> u64 {
        100
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Config {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self {
            access_token: String::new(),
            endpoint: Config::default_endpoint(),
            shutdown_timeout: Config::default_shutdown_timeout(),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = withAccessToken))]
    pub fn with_access_token(mut self, access_token: String) -> Self {
        self.access_token = access_token;
        self
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = withEndpoint))]
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = endpoint;
        self
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = withShutdownTimeout))]
    pub fn with_shutdown_timeout(mut self, shutdown_timeout: u64) -> Self {
        self.shutdown_timeout = shutdown_timeout;
        self
    }
}

impl Config {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn shutdown_timeout(&self) -> u64 {
        self.shutdown_timeout
    }
}
