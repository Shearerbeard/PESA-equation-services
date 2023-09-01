use equation::{
    client::{
        build_adder_client, build_divider_client, build_multiplier_client, build_subtractor_client,
    },
    config::Config,
};

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
    use equation::{parse::MathAST, proto::equation::CalculationRequest};

    #[actix_rt::test]
    async fn test_adder() {
        let config = Config::new();
        let mut client = build_adder_client(&config).await.unwrap();

        let request = tonic::Request::new(CalculationRequest {
            first_arg: serde_json::to_string(&MathAST::Value(1)).unwrap(),
            second_arg: serde_json::to_string(&MathAST::Value(2)).unwrap(),
        });

        let message = client.add(request).await.unwrap().into_inner();

        assert_eq!(message.result, 3);
    }

    #[actix_rt::test]
    async fn test_subtractor() {
        let config = Config::new();
        let mut client = build_subtractor_client(&config).await.unwrap();

        let request = tonic::Request::new(CalculationRequest {
            first_arg: serde_json::to_string(&MathAST::Value(5)).unwrap(),
            second_arg: serde_json::to_string(&MathAST::Value(2)).unwrap(),
        });

        let message = client.subtract(request).await.unwrap().into_inner();

        assert_eq!(message.result, 3);
    }

    #[actix_rt::test]
    async fn test_multiplier() {
        let config = Config::new();
        let mut client = build_multiplier_client(&config).await.unwrap();

        let request = tonic::Request::new(CalculationRequest {
            first_arg: serde_json::to_string(&MathAST::Value(5)).unwrap(),
            second_arg: serde_json::to_string(&MathAST::Value(2)).unwrap(),
        });

        let message = client.multiply(request).await.unwrap().into_inner();

        assert_eq!(message.result, 10);
    }

    #[actix_rt::test]
    async fn test_divider() {
        let config = Config::new();
        let mut client = build_divider_client(&config).await.unwrap();

        let request = tonic::Request::new(CalculationRequest {
            first_arg: serde_json::to_string(&MathAST::Value(4)).unwrap(),
            second_arg: serde_json::to_string(&MathAST::Value(2)).unwrap(),
        });

        let message = client.divide(request).await.unwrap().into_inner();

        assert_eq!(message.result, 2);
    }

    #[actix_rt::test]
    async fn test_e2e() {
        let config = Config::new();
        let mut client = build_subtractor_client(&config).await.unwrap();

        let request = tonic::Request::new(CalculationRequest {
            first_arg: serde_json::to_string(&MathAST::Divide(
                Box::new(MathAST::Multiply(
                    Box::new(MathAST::Add(
                        Box::new(MathAST::Value(3)),
                        Box::new(MathAST::Value(3)),
                    )),
                    Box::new(MathAST::Value(2)),
                )),
                Box::new(MathAST::Value(4)),
            ))
            .unwrap(),
            second_arg: serde_json::to_string(&MathAST::Value(2)).unwrap(),
        });

        let message = client.subtract(request).await.unwrap().into_inner();
        assert_eq!(message.result, 1);
    }
}
