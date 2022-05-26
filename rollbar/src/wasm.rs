use ::{serde_json::Value, std::collections::HashMap, wasm_bindgen::prelude::*};

use crate::{
    types::{Item, Level},
    Config, Transport,
};

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Instance {
    transport: Transport,
}

#[wasm_bindgen]
impl Instance {
    #[wasm_bindgen(js_name = "fromConfig")]
    pub fn from_config(input: JsValue) -> Result<Instance, JsValue> {
        let config: Config = input
            .into_serde()
            .map_err(|error| JsValue::from(format!("invalid config object: {}", error)))?;

        let transport = Transport::new(config)
            .map_err(|error| JsValue::from(format!("unable to create transport: {}", error)))?;

        Ok(Instance { transport })
    }

    pub fn log(&self, level: Level, message: &str, extra: JsValue) -> Result<(), JsValue> {
        let extra: Option<HashMap<String, Value>> = extra
            .into_serde()
            .map_err(|error| JsValue::from(format!("{}", error)))?;

        let item = Item::from((level, message, extra.unwrap_or_else(|| HashMap::new())));

        self.transport.send(item);

        Ok(())
    }

    pub fn debug(&self, message: &str, extra: JsValue) -> Result<(), JsValue> {
        self.log(Level::Debug, message, extra)
    }

    pub fn info(&self, message: &str, extra: JsValue) -> Result<(), JsValue> {
        self.log(Level::Info, message, extra)
    }

    pub fn warning(&self, message: &str, extra: JsValue) -> Result<(), JsValue> {
        self.log(Level::Warning, message, extra)
    }

    pub fn error(&self, message: &str, extra: JsValue) -> Result<(), JsValue> {
        self.log(Level::Error, message, extra)
    }

    pub fn critical(&self, message: &str, extra: JsValue) -> Result<(), JsValue> {
        self.log(Level::Critical, message, extra)
    }
}
