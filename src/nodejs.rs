use ::{neon::prelude::*, serde_json::Value, std::collections::HashMap};

use crate::{
    types::{Item, Level},
    Client, Config,
};

#[derive(Debug, Clone)]
pub struct Instance {
    client: Client,
}

impl Finalize for Instance {}

impl Instance {
    pub fn from_config(mut cx: FunctionContext) -> JsResult<JsBox<Instance>> {
        let input: Handle<JsValue> = cx.argument(0)?;

        let config: Config =
            neon_serde2::from_value(&mut cx, input).or_else(|e| cx.throw_error(e.to_string()))?;

        let client = Client::new(config).or_else(|e| cx.throw_error(e.to_string()))?;

        Ok(cx.boxed(Instance { client }))
    }

    pub fn shutdown(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Instance>, _>(&mut cx)?;
        instance.client.shutdown();
        Ok(cx.undefined())
    }

    pub fn log_with<'a>(
        instance: Handle<JsBox<Self>>,
        level: Level,
        start_arg_idx: i32,
        mut cx: FunctionContext<'a>,
    ) -> JsResult<'a, JsUndefined> {
        let message: Handle<JsString> = cx.argument(start_arg_idx)?;

        let extra: Option<Handle<JsValue>> = cx.argument_opt(start_arg_idx + 1);

        let extra: HashMap<String, Value> = if let Some(extra) = extra {
            neon_serde2::from_value(&mut cx, extra).or_else(|e| cx.throw_error(e.to_string()))?
        } else {
            HashMap::new()
        };

        let item = Item::from((level, message.value(&mut cx), extra));

        instance.client.send_item(item);

        Ok(cx.undefined())
    }

    pub fn log(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Instance>, _>(&mut cx)?;

        let level: Handle<JsString> = cx.argument(0)?;
        let level = Level::from(level.value(&mut cx));

        Self::log_with(instance, level, 1, cx)
    }

    pub fn debug(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Instance>, _>(&mut cx)?;

        Self::log_with(instance, Level::Debug, 0, cx)
    }

    pub fn info(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Instance>, _>(&mut cx)?;

        Self::log_with(instance, Level::Info, 0, cx)
    }

    pub fn warning(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Instance>, _>(&mut cx)?;

        Self::log_with(instance, Level::Warning, 0, cx)
    }

    pub fn error(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Instance>, _>(&mut cx)?;

        Self::log_with(instance, Level::Error, 0, cx)
    }

    pub fn critical(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Instance>, _>(&mut cx)?;

        Self::log_with(instance, Level::Critical, 0, cx)
    }
}

#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("fromConfig", Instance::from_config)?;
    cx.export_function("log", Instance::log)?;
    cx.export_function("debug", Instance::debug)?;
    cx.export_function("info", Instance::info)?;
    cx.export_function("warning", Instance::warning)?;
    cx.export_function("error", Instance::error)?;
    cx.export_function("critical", Instance::critical)?;
    cx.export_function("shutdown", Instance::shutdown)?;

    Ok(())
}
