use equation::{config::Config, proto::equation::divider_server::DividerServer};
use tokio::sync::mpsc;
use tonic::transport::Server;

use crate::server::DividerService;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Divider Init!");

    let (tx, mut rx) = mpsc::channel(100);
    let config = Config::new();
    let service = DividerService::new(&config, tx).await;

    Server::builder()
        .add_service(DividerServer::new(service))
        .serve_with_shutdown(config.divider_addr.parse()?, async {
            rx.recv().await;
            println!("Master shutdown request received by DividerServer");
        })
        .await?;

    println!("Divider Shutdown!");
    Ok(())
}
