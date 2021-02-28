//! A simple Rust library for (semi)anonymous SMS verification.
//!
//! Able to obtain a phone number for SMS verification and to read
//! messages from provided number.
//!
//! Currently supports only a single provider - `upmasked.com` with [awc]
//! client.
//!
//! ## Disclaimer
//! This crate is in an experimental stage, it is not stable and the
//! functionality can change any time; not recommended for use in
//! production systems.
//!
//! [awc]: https://docs.rs/awc/2.0.3/awc/
//! # Usage Example
//! Instantiates a new provider and retrieves latest message from 'Google' on
//! the first available number obtained from the default provider.
//!
//! ```
//! const ORIGIN: &str = "Google";
//!
//! let mut provider = provider::instantiate();
//! let num = provider.get_any_number().await.expect("valid number");
//! let msg = provider
//!     .get_latest_message_from(&num, ORIGIN)
//!     .await
//!     .expect("msg exists");
//!
//! println!(
//!     "Received message on {}: '{}'",
//!     msg.created_at.expect("timestamp"),
//!     msg.body
//! );
//! ```

#![warn(clippy::all, rust_2018_idioms)]

#[macro_use]
extern crate serde_derive;

pub mod provider;

pub type SmsServiceError = Box<dyn std::error::Error>;
type SmsServiceResult<T> = Result<T, SmsServiceError>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Resource could not be found")]
    NotFound,
}
