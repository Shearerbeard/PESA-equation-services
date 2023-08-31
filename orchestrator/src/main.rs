use client::{
    build_adder_client, build_divider_client, build_multiplier_client, build_subtractor_client,
};
use equation::config::Config;

mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();

    let mut adder_client = build_adder_client(&config).await?;
    let mut subtractor_client = build_subtractor_client(&config).await?;
    let mut multiplier_client = build_multiplier_client(&config).await?;
    let mut divider_client = build_divider_client(&config).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use equation::proto::equation::{
        CalculationRequest, CalculationResponse, Uuid
    };

    #[actix_rt::test] 
    async fn test_adder() {
        let config = Config::new();
        let mut client = build_adder_client(&config).await.unwrap();

        let id = "XXXXXXXXXXXXXXXX";

        let request = tonic::Request::new(CalculationRequest {
            id: Some(Uuid{ value: id.to_string() }),
            first_arg: 1,
            second_arg: 2,
        });

        let res = client.add(request).await.unwrap();

        println!("RES: {:#?}", res);
    }
}
