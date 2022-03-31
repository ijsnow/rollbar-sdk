use ::{
    env_logger::filter::{Builder as FilterBuilder, Filter},
    log::{Log, Metadata, Record},
};

use crate::{instance::Instance, types::Item, Config};

pub struct Logger {
    instance: Instance,
    filter: Filter,
}

impl Logger {
    pub fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let mut builder = FilterBuilder::new();

        if let Ok(ref filter) = std::env::var("RUST_LOG") {
            builder.parse(filter);
        }

        Self::new_with_filter(config, builder.build())
    }

    pub fn new_with_filter(
        config: Config,
        filter: impl Into<Filter>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Logger {
            instance: Instance::new(config)?,
            filter: filter.into(),
        })
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record<'_>) {
        let item = Item::from(record);
        self.instance.send_item(item);
    }

    fn flush(&self) {}
}
