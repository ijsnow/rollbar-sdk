#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("RuntimeCreation: {0}")]
    RuntimeCreation(std::io::Error),
    #[error("MessagesLock: could not obtain lock on message sender.")]
    MessagesLock,
    #[error("QueueDepthLock: could not obtain lock on queue depth.")]
    QueueDepthLock,
    #[error("MaxQueueDepthExceeded")]
    MaxQueueDepthExceeded,
    #[error("QueueDepthOutOfSync")]
    QueueDepthOutOfSync,
    #[error("TrySend: {0}")]
    TrySend(#[from] futures::channel::mpsc::TrySendError<Message>),
    #[error("Send: {0}")]
    Send(#[from] futures::channel::mpsc::SendError),
    #[error("Runtime: {0}")]
    Runtime(#[from] crate::runtime::Error),
    #[error("Http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("AccessDenied")]
    AccessDenied,
    #[error("RateLimited")]
    RateLimited,
    #[error("PayloadTooLarge")]
    PayloadTooLarge,
    #[error("MissingInfo: the api did not receive enough information for this item.")]
    MissingInfo,
    #[error("Shutdown with errors:\n {0}")]
    Shutdown(String),
}

use ::{
    futures::{channel::mpsc, sink::SinkExt, stream::StreamExt},
    reqwest::Client as HttpClient,
    reqwest::StatusCode,
    serde::{Deserialize, Serialize},
    std::sync::{Arc, Mutex},
};

use crate::{runtime, types::Item};

const QUEUE_DEPTH: usize = 50;
const API_ENDPOINT: &'static str = "api/1/item";

#[derive(Debug, Clone)]
pub struct Transport {
    messages: Arc<Mutex<mpsc::Sender<Message>>>,
    queue_depth: Arc<Mutex<u64>>,
    errors: Arc<Mutex<Vec<Error>>>,
    client: HttpClient,
    config: Config,
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Item(Item),
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize, typed_builder::TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "c", repr(C))]
pub struct Config {
    #[builder(setter(into))]
    pub access_token: String,
    #[builder(default = Config::default_uri())]
    #[serde(default = "Config::default_uri")]
    pub uri: String,
}

impl Config {
    pub fn default_uri() -> String {
        "https://api.rollbar.com".into()
    }
}

impl Transport {
    pub fn new(config: Config) -> Result<Self, Error> {
        let (messages, rcv_messages) = mpsc::channel(QUEUE_DEPTH as usize);

        let this = Self {
            messages: Arc::new(Mutex::new(messages)),
            queue_depth: Arc::new(Mutex::new(0)),
            errors: Arc::new(Mutex::new(vec![])),
            client: HttpClient::new(),
            config,
        };

        this.run(rcv_messages)?;

        Ok(this)
    }

    fn send_message(&self, message: Message) -> Result<(), Error> {
        if message != Message::Shutdown {
            let mut queue_depth = self.queue_depth.lock().map_err(|_| Error::QueueDepthLock)?;

            *queue_depth = queue_depth
                .checked_add(1)
                .ok_or(Error::MaxQueueDepthExceeded)?;
        }

        let sent = runtime::block_on(async move {
            let mut messages = self.messages.lock().map_err(|_| Error::MessagesLock)?;

            messages.send(message).await?;

            Ok(())
        })?;

        sent
    }

    pub fn send(&self, item: Item) -> Result<(), Error> {
        self.send_message(Message::Item(item))
    }

    pub fn shutdown(&self) -> Result<(), Error> {
        self.send_message(Message::Shutdown)?;

        while let Ok(queue_depth) = self.queue_depth.lock() {
            if *queue_depth == 0 {
                break;
            }
        }

        match self.errors.lock() {
            Ok(errors) if errors.len() > 0 => {
                let mut batch = String::new();

                for error in &errors[..] {
                    batch.push_str(&format!("{}\n", error));
                }

                Err(Error::Shutdown(batch))
            }
            _ => Ok(()),
        }
    }

    fn run(&self, mut messages: mpsc::Receiver<Message>) -> Result<(), Error> {
        let this = self.clone();

        let fut = async move {
            while let Some(message) = messages.next().await {
                match message {
                    Message::Item(item) => {
                        if let Err(error) = this.transport(item).await {
                            let mut errors = match this.errors.lock() {
                                Ok(errors) => errors,
                                _ => continue,
                            };

                            errors.push(error);
                        }

                        let mut queue_depth =
                            match this.queue_depth.lock().map_err(|_| Error::QueueDepthLock) {
                                Ok(queue_depth) => queue_depth,
                                Err(error) => {
                                    eprintln!("{}", error);
                                    continue;
                                }
                            };

                        *queue_depth =
                            match queue_depth.checked_sub(1).ok_or(Error::QueueDepthOutOfSync) {
                                Ok(next) => next,
                                Err(error) => {
                                    eprintln!("{}", error);
                                    continue;
                                }
                            };
                    }
                    Message::Shutdown => {
                        messages.close();
                    }
                }
            }
        };

        runtime::spawn(fut)?;

        Ok(())
    }

    async fn transport(&self, item: Item) -> Result<(), Error> {
        let result = self
            .client
            .post(&format!("{}/{}", &self.config.uri, API_ENDPOINT))
            .header("X-Rollbar-Access-Token", &self.config.access_token)
            .json(&item)
            .send()
            .await?;

        match result.status() {
            // TODO: truncate payload... but truncate what?
            StatusCode::PAYLOAD_TOO_LARGE => Err(Error::PayloadTooLarge),
            // TODO: wait for x to retry.
            StatusCode::TOO_MANY_REQUESTS => Err(Error::RateLimited),
            // TODO: don't send requests until reconfigured with new token?
            StatusCode::FORBIDDEN => Err(Error::AccessDenied),
            // TODO: possibly unnecessary if library guaranteed to not send invalid requests.
            StatusCode::UNPROCESSABLE_ENTITY => Err(Error::MissingInfo),
            _ => Ok(()),
        }
    }
}

#[test]
fn shutdown_waits_for_items() {
    use crate::{runtime, types::*};

    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    let mock_server = runtime::block_on(async { MockServer::start().await }).unwrap();

    let expected_count = QUEUE_DEPTH * 2;

    runtime::block_on(async {
        Mock::given(method("POST"))
            .and(path(API_ENDPOINT))
            .respond_with(ResponseTemplate::new(200))
            .expect(expected_count as u64)
            .mount(&mock_server)
            .await
    })
    .unwrap();

    let items = (0..expected_count)
        .map(|i| Item::from((Level::Info, format!("{}", i), Default::default())));

    let config = Config::builder()
        .uri(mock_server.uri())
        .access_token("abc")
        .build();

    let transport = Transport::new(config).unwrap();

    for item in items {
        transport.send(item).unwrap();
    }

    transport.shutdown().unwrap();
}
