use ::{reqwest::Client as HttpClient, std::collections::HashMap};

#[cfg(target_arch = "wasm32")]
use ::{serde_json::Value, wasm_bindgen::prelude::*};

use crate::{
    types::{Item, Level},
    Config,
};

#[derive(Debug, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Instance {
    config: Config,
    client: HttpClient,
}

impl Instance {
    pub fn send_item(&self, item: Item) {
        let this = self.clone();

        let fut = async move {
            if let Err(_err) = this.send(item).await {
                // silently ignore to be forgiving
            }
        };

        #[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Instance {
    #[wasm_bindgen(js_name = "initialize")]
    pub fn from_js_value(input: JsValue) -> Result<Instance, JsValue> {
        let config: Config = input
            .into_serde()
            .map_err(|_| JsValue::from("invalid config object"))?;

        Ok(Instance {
            config,
            client: HttpClient::new(),
        })
    }

    pub fn log(&self, level: Level, message: &str, extra: JsValue) -> Result<(), JsValue> {
        let extra: Option<HashMap<String, Value>> = extra
            .into_serde()
            .map_err(|_| JsValue::from("invalid extra data"))?;

        let item = Item::from((level, message, extra.unwrap_or_else(|| HashMap::new())));

        self.send_item(item);

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

#[cfg(feature = "node")]
use neon::{prelude::*, types::JsBox};

#[cfg(feature = "node")]
impl Instance {
    pub fn from_js_context(mut cx: FunctionContext) -> JsResult<JsBox<Instance>> {
        let cfg_handle = cx.argument::<JsObject>(0)?;

        let access_token: Handle<JsString> = cfg_handle.get(&mut cx, "accessToken")?;
        let endpoint: Option<Handle<JsString>> = cfg_handle.get_opt(&mut cx, "endpoint")?;

        let config = Config {
            endpoint: endpoint
                .map(|handle| handle.value())
                .unwrap_or_else(|_| Config::default_endpoint()),
            access_token,
        };

        Ok(cx.boxed(Instance {
            config,
            client: HttpClient::new(),
        }))
    }

    pub fn log(&self, level: Level, message: &str, extra: JsValue) -> Result<(), JsValue> {
        let extra: Option<HashMap<String, Value>> = extra
            .into_serde()
            .map_err(|_| JsValue::from("invalid extra data"))?;

        let item = Item::from((level, message, extra.unwrap_or_else(|| HashMap::new())));

        self.send_item(item);

        Ok(())
    }

    /*
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
    */
}
