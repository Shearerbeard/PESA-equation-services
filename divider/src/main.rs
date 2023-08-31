use equation::{config::Config, proto::equation::divider_server::DividerServer};
use tonic::transport::Server;

use crate::server::DividerService;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Divider Init!");

    let config = Config::new();
    let service = DividerService::default();

    Server::builder()
        .add_service(DividerServer::new(service))
        .serve(config.divider_addr.parse()?)
        .await?;

    println!("Divider Shutdown!");
    Ok(())
}
