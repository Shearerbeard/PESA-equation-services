use equation::{config::Config, proto::equation::multiplier_server::MultiplierServer};
use tonic::transport::Server;

use crate::server::MultiplierService;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Multiplier Init!");

    let config = Config::new();
    let service = MultiplierService::default();

    Server::builder()
        .add_service(MultiplierServer::new(service))
        .serve(config.adder_addr.parse()?)
        .await?;

    println!("Multiplier Shutdown!");
    Ok(())
}
