#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error sending item {0}")]
    SendItem(#[from] futures::channel::mpsc::TrySendError<Item>),
    #[error("Unable to create runtime {0}")]
    RuntimeCreation(std::io::Error),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("RuntimeLock: unable to obtain lock on runtime.")]
    RuntimeLock,
    #[error("SenderLock: unable to obtain lock on sender.")]
    SenderLock,
}

use ::{
    futures::{channel::mpsc, StreamExt},
    reqwest::Client as HttpClient,
    std::sync::{Arc, Mutex},
};

#[cfg(not(target_arch = "wasm32"))]
use ::std::time::Duration;

use crate::{types::Item, Config};

static QUEUE_DEPTH: usize = 10;

#[derive(Clone, Debug)]
pub struct Client {
    config: Config,
    client: HttpClient,
    item_sender: Arc<Mutex<mpsc::Sender<Item>>>,
}

impl Client {
    pub fn new(config: Config) -> Result<Self, Error> {
        let (item_sender, items) = mpsc::channel(QUEUE_DEPTH);

        let this = Self {
            config,
            item_sender: Arc::new(Mutex::new(item_sender)),
            client: HttpClient::new(),
        };

        this.run(items)?;

        Ok(this)
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    fn run(&self, mut items: mpsc::Receiver<Item>) -> Result<(), Error> {
        let this = self.clone();

        let future = async move {
            while let Some(item) = items.next().await {
                let client = this.client.clone();
                let config = this.config.clone();

                let inner_future = async { send_item(client, config, item).await };

                #[cfg(target_arch = "wasm32")]
                wasm_bindgen_futures::spawn_local(future);

                #[cfg(not(target_arch = "wasm32"))]
                inner_runtime.spawn(inner_future);
            }

            #[cfg(not(target_arch = "wasm32"))]
            inner_runtime.shutdown_timeout(Duration::from_millis(this.config.shutdown_timeout()));
        };

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(future);

        #[cfg(not(target_arch = "wasm32"))]
        outer_runtime.spawn(future);

        Ok(())
    }

    pub fn send_item(&self, item: Item) -> Result<(), Error> {
        let mut sender = self.item_sender.lock().map_err(|_| Error::SenderLock)?;

        sender.try_send(item)?;

        Ok(())
    }

    pub fn shutdown(self) -> Result<(), Error> {
        let mut sender = self.item_sender.lock().map_err(|_| Error::SenderLock)?;

        sender.close_channel();

        Ok(())
    }

    async fn send(&self, item: Item) {
        if let Err(_error) = self
            .client
            .post(self.config.endpoint())
            .header("X-Rollbar-Access-Token", self.config.access_token())
            .json(&item)
            .send()
            .await
        {
            // TODO: handle api errors (retry, etc)
        }
    }
}

async fn send_item(client: HttpClient, config: Config, item: Item) {
    if let Err(_error) = client
        .post(config.endpoint())
        .header("X-Rollbar-Access-Token", config.access_token())
        .json(&item)
        .send()
        .await
    {
        // TODO: handle api errors (retry, etc)
    }
}
