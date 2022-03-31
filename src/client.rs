use ::{async_trait::async_trait, std::fmt::Debug};

use crate::types::Item;

pub type ClientError = Box<dyn std::error::Error>;

#[async_trait]
pub trait Client: Debug + Send + Sync {
    async fn send(&self, item: Item) -> Result<(), ClientError>;
}
