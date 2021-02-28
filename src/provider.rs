use chrono::{DateTime, Utc};

pub mod upmasked;

use self::upmasked::UpmaskedProvider;
use crate::{Error, SmsServiceError, SmsServiceResult};

/// Number obtained from (or provided to) a provider.
#[derive(Clone, Debug, Deserialize)]
pub struct Number {
    pub number: String,
    pub country: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

/// SMS which has been obtained from a provider.
#[derive(Clone, Debug, Deserialize)]
pub struct Message {
    pub body: String,
    pub originator: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

/// A convenience function to instantiate the default provider.
pub fn instantiate() -> Box<dyn ProviderClient> {
    Box::new(UpmaskedProvider::default())
}

/// Function to instantiate a provider.
pub fn instantiate_with_provider<T: ProviderClient + Default>() -> T {
    T::default()
}

/// The main trait for an SMS verification provider.
///
/// Any provider that is to be used with the library should implement the
/// the `ProviderClient` trait. Only methods to list phone numbers and to
/// retrieves messages are required, other methods have default implementation.
#[async_trait::async_trait(?Send)]
pub trait ProviderClient {
    /// Method to list all available phone numbers from a provider.
    async fn get_all_numbers(&mut self) -> SmsServiceResult<Vec<Number>>;

    /// Method to retrieve messages recieved on a specified `number`.
    async fn get_messages(
        &mut self,
        number: &Number,
    ) -> SmsServiceResult<Vec<Message>>;

    /// Retrieves any available number for use.
    async fn get_any_number(&mut self) -> SmsServiceResult<Number> {
        let numbers = self.get_all_numbers().await?;

        if let Some(n) = numbers.first() {
            return Ok(n.to_owned());
        } else {
            return Err(SmsServiceError::from(Error::NotFound));
        }
    }

    /// Tries to obtain a number for specified country. Format of `country` can
    /// be provider-specific; normally the two-letter country code (ISO
    /// 3166-1 alpha-2) should be asssumed. Returns `NotFound` error if no
    /// number is available.
    async fn get_number_for_country(
        &mut self,
        country: &str,
    ) -> SmsServiceResult<Number> {
        let numbers = self.get_all_numbers().await?;

        let num_with_country = numbers.into_iter().find(|n| {
            if let Some(c) = n.country.as_ref() {
                c == country
            } else {
                false
            }
        });

        if let Some(n) = num_with_country {
            return Ok(n);
        } else {
            return Err(SmsServiceError::from(Error::NotFound));
        }
    }

    /// Retrieves last message recieved on `number`. Returns `NotFound`
    /// error if no message is available.
    async fn get_latest_message(
        &mut self,
        number: &Number,
    ) -> SmsServiceResult<Message> {
        let mut msgs = self.get_messages(number).await?;
        msgs.sort_by(|m1, m2| m2.created_at.cmp(&m1.created_at));

        if let Some(m) = msgs.first() {
            return Ok(m.to_owned());
        } else {
            return Err(SmsServiceError::from(Error::NotFound));
        }
    }

    /// Retrieves last message recieved on `number` from `originator`.
    /// Returns `NotFound` error if no message is available.
    async fn get_latest_message_from(
        &mut self,
        number: &Number,
        originator: &str,
    ) -> SmsServiceResult<Message> {
        let mut msgs = self.get_messages(number).await?;
        msgs.sort_by(|m1, m2| m2.created_at.cmp(&m1.created_at));

        let msg_with_originator = msgs.into_iter().find(|m| {
            if let Some(o) = m.originator.as_ref() {
                o == originator
            } else {
                false
            }
        });

        if let Some(m) = msg_with_originator {
            return Ok(m);
        } else {
            return Err(SmsServiceError::from(Error::NotFound));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod mock;

    #[actix_rt::test]
    async fn test_get_latest_msg_from() {
        let mut p = mock::MockProvider::default();

        let num = p.get_any_number().await;
        assert!(num.is_ok());

        let num = num.expect("ok");
        assert_eq!(num.number, mock::TEST_NUM);
        assert_eq!(num.country, Some(String::from(mock::TEST_COUNTRY)));
        assert!(num.created_at.is_some());

        let msg = p.get_latest_message_from(&num, mock::TEST_ORIG).await;
        assert!(msg.is_ok());

        let msg = msg.expect("ok");
        assert_eq!(msg.originator, Some(String::from(mock::TEST_ORIG)));
        assert!(msg.created_at.is_some());
    }
}
