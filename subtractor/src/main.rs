use equation::{config::Config, proto::equation::subtractor_server::SubtractorServer};
use tokio::sync::mpsc;
use tonic::transport::Server;

use crate::server::SubtractorService;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Subtractor Init!");

    let (tx, mut rx) = mpsc::channel(100);
    let config = Config::new();
    let service = SubtractorService::new(&config, tx).await;

    Server::builder()
        .add_service(SubtractorServer::new(service))
        .serve_with_shutdown(config.subtractor_addr.parse()?, async {
            rx.recv().await;
            println!("Master shutdown request received by SubtractorServer");
        })
        .await?;

    println!("Subtractor Shutdown!");
    Ok(())
}
