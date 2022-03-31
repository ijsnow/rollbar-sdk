mod client;
mod config;
mod instance;
// mod logger;
mod types;

pub use self::{config::Config, instance::Instance};

/*
use types::Item;

pub fn set_panic_hook(instance: Instance) {
    let instance = instance.clone();

    std::panic::set_hook(Box::new(move |info| {
        let item = Item::from(info);
        instance.send_item(item);
    }));
}

pub fn set_logger_with_filter(
    config: Config,
    filter: env_logger::filter::Filter,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::logger::Logger;
    use once_cell::sync::OnceCell;

    static LOGGER: OnceCell<Logger> = OnceCell::new();

    log::set_logger(match LOGGER.get() {
        Some(logger) => logger,
        None => {
            let logger = logger::Logger::new_with_filter(config, filter)?;
            LOGGER.get_or_init(|| logger)
        }
    })?;

    Ok(())
}
*/
