use client::{
    build_adder_client, build_divider_client, build_multiplier_client, build_subtractor_client,
};
use equation::config::Config;

mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();

    let mut adder_client = build_adder_client(&config);
    let mut subtractor_client = build_subtractor_client(&config);
    let mut multiplier_client = build_multiplier_client(&config);
    let mut divider_client = build_divider_client(&config);

    Ok(())
}
