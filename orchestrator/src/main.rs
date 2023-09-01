use equation::{
    client::{
        build_adder_client, build_divider_client, build_multiplier_client, build_subtractor_client,
    },
    config::Config,
    parse::{test_value, MathAST},
    proto::equation::{
        adder_client::AdderClient, divider_client::DividerClient,
        multiplier_client::MultiplierClient, subtractor_client::SubtractorClient,
        CalculationRequest, Empty,
    },
    server::wait_for_ctrl_c,
};
use tokio::{spawn, sync::mpsc};
use tonic::{transport::Channel, Status};

#[derive(Clone)]
struct Clients {
    adder_client: AdderClient<Channel>,
    subtractor_client: SubtractorClient<Channel>,
    multiplier_client: MultiplierClient<Channel>,
    divider_client: DividerClient<Channel>,
}

impl Clients {
    pub(crate) async fn new(config: &Config) -> Self {
        Self {
            adder_client: build_adder_client(&config).await.expect("client connect"),
            subtractor_client: build_subtractor_client(&config)
                .await
                .expect("client connect"),
            multiplier_client: build_multiplier_client(&config)
                .await
                .expect("cllinet connect"),
            divider_client: build_divider_client(&config).await.expect("client connect"),
        }
    }

    pub(crate) async fn shutdown(&mut self) {
        let message = Empty {};
        println!("Sending term command to adder");
        let _ = self.adder_client.term(message.clone()).await;

        println!("Sending term command to subtractor");
        let _ = self.subtractor_client.term(message.clone()).await;

        println!("Sending term command to multiplier");
        let _ = self.multiplier_client.term(message.clone()).await;

        println!("Sending term command to divider");
        let _ = self.divider_client.term(message).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    let mut clients = Clients::new(&config).await;

    let (signal_tx, mut signal_rx) = mpsc::channel(100);
    spawn(wait_for_ctrl_c(signal_tx));

    // Test distributed equation with AST from exercise ( ( (3 + 3)*2) /4) – 2 = X
    let ast = test_value();
    let res = run_equation(&mut clients, ast).await?;
    println!("The result of your equation is: {:?}", res);

    println!("Blocking on signal for CTRL-C");
    signal_rx.recv().await;
    println!("RECEIVED CTRL-C - SHUTTING DOWN SERVICES");
    clients.shutdown().await;

    Ok(())
}

async fn run_equation(clients: &mut Clients, ast: MathAST) -> Result<i32, Status> {
    match ast {
        MathAST::Value(v) => Ok(v),
        MathAST::Add(first, second) => {
            let request = tonic::Request::new(CalculationRequest {
                first_arg: serde_json::to_string(&first).unwrap(),
                second_arg: serde_json::to_string(&second).unwrap(),
            });

            let message = clients.adder_client.add(request).await?.into_inner();
            Ok(message.result)
        }
        MathAST::Subtract(first, second) => {
            let request = tonic::Request::new(CalculationRequest {
                first_arg: serde_json::to_string(&first).unwrap(),
                second_arg: serde_json::to_string(&second).unwrap(),
            });

            let message = clients
                .subtractor_client
                .subtract(request)
                .await?
                .into_inner();
            Ok(message.result)
        }
        MathAST::Multiply(first, second) => {
            let request = tonic::Request::new(CalculationRequest {
                first_arg: serde_json::to_string(&first).unwrap(),
                second_arg: serde_json::to_string(&second).unwrap(),
            });

            let message = clients
                .multiplier_client
                .multiply(request)
                .await?
                .into_inner();
            Ok(message.result)
        }
        MathAST::Divide(first, second) => {
            let request = tonic::Request::new(CalculationRequest {
                first_arg: serde_json::to_string(&first).unwrap(),
                second_arg: serde_json::to_string(&second).unwrap(),
            });

            let message = clients.divider_client.divide(request).await?.into_inner();
            Ok(message.result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut clients = Clients::new(&config).await;
        let ast = test_value();
        let res = run_equation(&mut clients, ast).await.expect("Result");
        assert_eq!(res, 1);
    }
}
