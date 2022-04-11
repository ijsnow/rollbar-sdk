mod client;
mod config;
mod handle;
mod types;

pub use self::{client::Client, config::Config, handle::Handle};

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

#[cfg(feature = "nodejs")]
mod nodejs;

#[cfg(feature = "nodejs")]
pub use nodejs::*;

//#[cfg(feature = "nodejs")]
//use neon::prelude::*;

//#[cfg(feature = "nodejs")]
//#[neon::main]
//pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
//cx.export_function("fromConfig", Instance::from_config)?;
//cx.export_function("log", Instance::log)?;
//cx.export_function("debug", Instance::debug)?;
//cx.export_function("info", Instance::info)?;
//cx.export_function("warning", Instance::warning)?;
//cx.export_function("error", Instance::error)?;
//cx.export_function("critical", Instance::critical)?;

//Ok(())
//}
