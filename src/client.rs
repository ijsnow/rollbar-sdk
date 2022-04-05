use reqwest::Client as HttpClient;

use crate::{types::Item, Config};

#[derive(Clone, Debug)]
pub struct Client {
    config: Config,
    client: HttpClient,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: HttpClient::new(),
        }
    }

    pub fn send_item(&self, item: Item) {
        let this = self.clone();

        let future = async move {
            if let Err(_err) = this.send(item).await {
                // silently ignore to be forgiving
            }
        };

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(future);

        #[cfg(feature = "nodejs")]
        tokio::runtime::Runtime::new().unwrap().block_on(future);
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
