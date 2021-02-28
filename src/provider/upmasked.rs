use awc::Client;
use url::Url;

use crate::{
    provider::{Message, Number, ProviderClient, SMSServiceError},
    SMSServiceResult,
};

const UPMASKED_URL: &str = "https://upmasked.com/";

/// Implements provider for Upmasked
pub struct UpmaskedProvider {
    client: awc::Client,
    base_url: Url,
}

impl UpmaskedProvider {
    /// Builds new Upmasked provider from provided `awc::Client`. Unless
    /// there is a reason for having custom client, use the `default()`
    /// to instantiate new provider with the default client.
    pub fn from_client(client: awc::Client) -> UpmaskedProvider {
        UpmaskedProvider {
            client,
            base_url: Url::parse(UPMASKED_URL).expect("valid URL"),
        }
    }
}

impl Default for UpmaskedProvider {
    fn default() -> Self {
        UpmaskedProvider {
            client: Client::default(),
            base_url: Url::parse(UPMASKED_URL).expect("valid URL"),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl ProviderClient for UpmaskedProvider {
    async fn get_all_numbers(&mut self) -> SMSServiceResult<Vec<Number>> {
        let url = self.base_url.join("/api/sms/numbers").expect("valid URL");
        let mut res = self.client.get(url.as_str()).send().await?;
        let json: Vec<Number> = res.json().await?;
        Ok(json)
    }

    async fn get_messages(&mut self, number: &Number) -> Result<Vec<Message>, SMSServiceError> {
        let url = self
            .base_url
            .join(format!("/api/sms/messages/{}", number.number).as_str())
            .expect("valid URL");
        let mut res = self.client.get(url.as_str()).send().await?;
        let json: Vec<Message> = res.json().await?;
        Ok(json)
    }
}
