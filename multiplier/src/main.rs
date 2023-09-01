use equation::{config::Config, proto::equation::multiplier_server::MultiplierServer};
use tokio::sync::mpsc;
use tonic::transport::Server;

use crate::server::MultiplierService;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Multiplier Init!");

    let (tx, mut rx) = mpsc::channel(100);
    let config = Config::new();
    let service = MultiplierService::new(&config, tx).await;

    Server::builder()
        .add_service(MultiplierServer::new(service))
        .serve_with_shutdown(config.multiplier_addr.parse()?, async {
            rx.recv().await;
            println!("Master shutdown request received by MultiplierServer");
        })
        .await?;

    println!("Multiplier Shutdown!");
    Ok(())
}
