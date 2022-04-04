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
}

impl Config {
    pub fn default_endpoint() -> String {
        "https://api.rollbar.com/api/1/item".into()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Config {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self {
            access_token: String::new(),
            endpoint: "https:://api.rollbar.com/api/1/item".into(),
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
}

impl Config {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}
