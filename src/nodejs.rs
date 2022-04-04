use ::{neon::prelude::*, serde_json::Value, std::collections::HashMap};

use crate::{
    types::{Item, Level},
    Client, Config,
};

#[derive(Debug, Clone)]
pub struct Instance {
    client: Client,
}

impl Instance {
    pub fn from_config(mut cx: FunctionContext) -> JsResult<JsBox<Instance>> {
        let input: Handle<JsObject> = cx.argument(0)?;

        let access_token: JsValue<JsString> = input.get(&mut cx, "accessToken")?;
        let endpoint: Option<JsValue<JsString>> = input.get_opt(&mut cx, "endpoint")?;

        let config = Config {
            access_token: access_token.value(),
            endpoint: endpoint
                .map(JsString::value)
                .unwrap_or_else(Config::default_endpoint),
        };

        Ok(cx.boxed(Instance {
            client: Client::new(config),
        }))
    }

    /*
    pub fn log(&self, level: Level, message: &str, extra: JsValue) {
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
    */
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("fromConfig", Instance::from_config)?;
    Ok(())
}
