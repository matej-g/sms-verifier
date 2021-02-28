use crate::{
    provider::{Message, Number, ProviderClient},
    SMSServiceResult,
};

pub(crate) const TEST_NUM: &str = "123456789";
pub(crate) const TEST_COUNTRY: &str = "US";
pub(crate) const TEST_MSG: &str = "Hello there";
pub(crate) const TEST_ORIG: &str = "Test";
#[derive(Default)]
pub(crate) struct MockProvider {}

#[async_trait::async_trait(?Send)]
impl ProviderClient for MockProvider {
    async fn get_all_numbers(&mut self) -> SMSServiceResult<Vec<Number>> {
        return Ok(vec![Number {
            number: String::from(TEST_NUM),
            country: Some(String::from(TEST_COUNTRY)),
            created_at: Some(chrono::Utc::now()),
        }]);
    }

    async fn get_messages(&mut self, _: &Number) -> SMSServiceResult<Vec<Message>> {
        return Ok(vec![Message {
            body: String::from(TEST_MSG),
            originator: Some(String::from(TEST_ORIG)),
            created_at: Some(chrono::Utc::now()),
        }]);
    }
}
