use ::{reqwest::Client as HttpClient, std::collections::HashMap};

use ::{serde_json::Value, wasm_bindgen::prelude::*};

use crate::{
    types::{Item, Level},
    Config,
};

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Instance {
    config: Config,
    client: HttpClient,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
impl Instance {
    #[wasm_bindgen(js_name = "fromConfig")]
    pub fn from_config(input: JsValue) -> Result<Instance, JsValue> {
        let config: Config = input
            .into_serde()
            .map_err(|_| JsValue::from("invalid config object"))?;

        Ok(Instance {
            config,
            client: HttpClient::new(),
        })
    }

    pub fn log(&self, level: Level, message: &str, extra: JsValue) {
        log(&format!("{:?}", extra));

        let extra: Option<HashMap<String, Value>> = extra.into_serde().expect("to work");

        let item = Item::from((level, message, extra.unwrap_or_else(|| HashMap::new())));

        self.send_item(item);
    }

    pub fn debug(&self, message: &str, extra: JsValue) {
        self.log(Level::Debug, message, extra)
    }

    pub fn info(&self, message: &str, extra: JsValue) {
        self.log(Level::Info, message, extra)
    }

    pub fn warning(&self, message: &str, extra: JsValue) {
        self.log(Level::Warning, message, extra)
    }

    pub fn error(&self, message: &str, extra: JsValue) {
        self.log(Level::Error, message, extra)
    }

    pub fn critical(&self, message: &str, extra: JsValue) {
        self.log(Level::Critical, message, extra)
    }
}

impl Instance {
    pub fn send_item(&self, item: Item) {
        let this = self.clone();

        let fut = async move {
            if let Err(_err) = this.send(item).await {
                // silently ignore to be forgiving
            }
        };

        wasm_bindgen_futures::spawn_local(fut);
    }

    async fn send(&self, item: Item) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: handle api errors (retry, etc)
        self.client
            .post(self.config.endpoint())
            .header("X-Rollbar-Access-Token", self.config.access_token())
            .json(&item)
            .send()
            .await?;

        Ok(())
    }
}
