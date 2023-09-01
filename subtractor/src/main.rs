use equation::{config::Config, proto::equation::subtractor_server::SubtractorServer};
use tonic::transport::Server;

use crate::server::SubtractorService;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Subtractor Init!");

    let config = Config::new();
    let service = SubtractorService::new(&config).await;

    Server::builder()
        .add_service(SubtractorServer::new(service))
        .serve(config.subtractor_addr.parse()?)
        .await?;

    println!("Subtractor Shutdown!");
    Ok(())
}
