use equation::{
    client::{build_divider_client, build_multiplier_client, build_subtractor_client},
    config::Config,
    proto::equation::adder_server::AdderServer,
};
use server::AdderService;
use tonic::transport::Server;

mod error;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Adder Init!");

    let config = Config::new();
    let subtractor_client = build_subtractor_client(&config).await?;
    let multiplier_client = build_multiplier_client(&config).await?;
    let divider_client = build_divider_client(&config).await?;
    let service = AdderService::new(subtractor_client, multiplier_client, divider_client);

    Server::builder()
        .add_service(AdderServer::new(service))
        .serve(config.adder_addr.parse()?)
        .await?;

    println!("Adder Shutdown!");
    Ok(())
}
