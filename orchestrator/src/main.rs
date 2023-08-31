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
    use equation::proto::equation::{CalculationRequest, CalculationResponse, Uuid};
    use tonic::Response;

    #[actix_rt::test]
    async fn test_adder() {
        let config = Config::new();
        let mut client = build_adder_client(&config).await.unwrap();

        let id = "AXXXXXXXXXXXXXXXX";

        let request = tonic::Request::new(CalculationRequest {
            id: Some(Uuid {
                value: id.to_string(),
            }),
            first_arg: 1,
            second_arg: 2,
        });

        let message = client.add(request).await.unwrap().into_inner();

        assert_eq!(message.id.unwrap().value, id);
        assert_eq!(message.result, 3);
    }

    #[actix_rt::test]
    async fn test_subtractor() {
        let config = Config::new();
        let mut client = build_subtractor_client(&config).await.unwrap();

        let id = "SXXXXXXXXXXXXXXXX";

        let request = tonic::Request::new(CalculationRequest {
            id: Some(Uuid {
                value: id.to_string(),
            }),
            first_arg: 5,
            second_arg: 2,
        });

        let message = client.subtract(request).await.unwrap().into_inner();

        assert_eq!(message.id.unwrap().value, id);
        assert_eq!(message.result, 3);
    }

    #[actix_rt::test]
    async fn test_multiplier() {
        let config = Config::new();
        let mut client = build_multiplier_client(&config).await.unwrap();

        let id = "MXXXXXXXXXXXXXXXX";

        let request = tonic::Request::new(CalculationRequest {
            id: Some(Uuid {
                value: id.to_string(),
            }),
            first_arg: 5,
            second_arg: 2,
        });

        let message = client.multiply(request).await.unwrap().into_inner();

        assert_eq!(message.id.unwrap().value, id);
        assert_eq!(message.result, 10);
    }

    #[actix_rt::test]
    async fn test_divider() {
        let config = Config::new();
        let mut client = build_divider_client(&config).await.unwrap();

        let id = "DXXXXXXXXXXXXXXXX";

        let request = tonic::Request::new(CalculationRequest {
            id: Some(Uuid {
                value: id.to_string(),
            }),
            first_arg: 4,
            second_arg: 2,
        });

        let message = client.divide(request).await.unwrap().into_inner();

        assert_eq!(message.id.unwrap().value, id);
        assert_eq!(message.result, 2);
    }
}
