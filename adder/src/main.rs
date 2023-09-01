use equation::{config::Config, proto::equation::adder_server::AdderServer};
use server::AdderService;
use tokio::sync::mpsc;
use tonic::transport::Server;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Adder Init!");

    let (tx, mut rx) = mpsc::channel(100);
    let config = Config::new();
    let service = AdderService::new(&config, tx.clone()).await;

    Server::builder()
        .add_service(AdderServer::new(service))
        .serve_with_shutdown(config.adder_addr.parse()?, async {
            rx.recv().await;
            println!("Master shutdown request received by AdderServer");
        })
        .await?;

    println!("Adder Shutdown!");
    Ok(())
}
