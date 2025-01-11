//! As a blog visitor, I want to subscribe to the newsletter, So that I can receive email updates when new content is published on blog.
//! As a blog author, I want to send an email to all my subscribers, So that I can modify them when new content is published.

use std::net::TcpListener;

use zero2production::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
