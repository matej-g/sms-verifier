use sms_verifier::{provider, Error};

#[actix_rt::main]
async fn main() {
    const ORIGIN: &str = "Google";

    let mut provider = provider::instantiate();
    let num = provider.get_any_number().await.expect("valid number");
    let msg = provider.get_latest_message_from(&num, ORIGIN).await;

    match msg {
        Ok(m) => println!(
            "Last message recieved on {}: '{}'",
            m.created_at.expect("timestamp"),
            m.body
        ),
        Err(e) => {
            if let Some(service_err) = e.downcast_ref::<Error>() {
                match *service_err {
                    Error::NotFound => {
                        println!("Sorry, no message from {} found.", ORIGIN);
                        return;
                    },
                }
            }

            println!("Error occured: {}", e)
        },
    }
}
