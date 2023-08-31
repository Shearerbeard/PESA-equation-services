use equation::{config::Config, proto::equation::adder_server::AdderServer};
use tonic::transport::Server;
use server::AdderService;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Adder Init!");

    let config = Config::new();
    let service = AdderService ::default();

    println!("Config: {:#?}", config);

    Server::builder()
        .add_service(AdderServer::new(service))
        .serve(config.adder_addr.parse()?)
        .await?;

    println!("Adder Shutdown!");
    Ok(())
}
