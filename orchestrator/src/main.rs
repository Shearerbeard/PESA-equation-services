use equation::{
    client::{
        build_adder_client, build_divider_client, build_multiplier_client, build_subtractor_client,
    },
    config::Config,
    parse::{test_value, MathAST},
    proto::equation::{
        adder_client::AdderClient, divider_client::DividerClient,
        multiplier_client::MultiplierClient, subtractor_client::SubtractorClient,
        CalculationResponse, Empty,
    },
    server::wait_for_ctrl_c,
};
use tokio::{spawn, sync::mpsc};
use tonic::{transport::Channel, Response, Status};

#[derive(Clone)]
struct Clients {
    adder_client: AdderClient<Channel>,
    subtractor_client: SubtractorClient<Channel>,
    multiplier_client: MultiplierClient<Channel>,
    divider_client: DividerClient<Channel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();

    let ast = test_value();

    let mut adder_client = build_adder_client(&config).await?;
    let mut subtractor_client = build_subtractor_client(&config).await?;
    let mut multiplier_client = build_multiplier_client(&config).await?;
    let mut divider_client = build_divider_client(&config).await?;

    let mut clients = Clients {
        adder_client,
        subtractor_client,
        multiplier_client,
        divider_client,
    };

    let (signal_tx, mut signal_rx) = mpsc::channel(100);
    spawn(wait_for_ctrl_c(signal_tx));

    println!("Blocking on signal for CTRL-C");
    signal_rx.recv().await;
    println!("RECEIVED CTRL-C - SHUTTING DOWN");
    let message = Empty {};
    let _ = clients.adder_client.term(message.clone()).await;
    let _ = clients.subtractor_client.term(message.clone()).await;
    let _ = clients.multiplier_client.term(message.clone()).await;
    let _ = clients.divider_client.term(message).await;

    Ok(())
}

async fn run_distributed_equation(
    clients: &mut Clients,
    ast: MathAST,
) -> Result<Response<CalculationResponse>, Status> {
    match ast {
        MathAST::Value(_) => ast,
        MathAST::Add(_, _) => {
            todo!()
        }
        MathAST::Subtract(_, _) => {
            todo!()
        }
        MathAST::Multiply(_, _) => {
            todo!()
        }
        MathAST::Divide(_, _) => {
            todo!()
        }
    }
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
