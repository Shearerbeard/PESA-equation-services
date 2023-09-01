use std::sync::Arc;

use async_recursion::async_recursion;
use async_trait::async_trait;
use equation::{
    client::{build_adder_client, build_multiplier_client, build_subtractor_client},
    config::Config,
    parse::{MathAST, MathASTEvaluator},
    proto::equation::{
        adder_client::AdderClient, divider_server::Divider, multiplier_client::MultiplierClient,
        subtractor_client::SubtractorClient, CalculationRequest, CalculationResponse, Empty,
    },
    server::Error,
};
use tokio::sync::{mpsc::Sender, Mutex};
use tonic::{transport::Channel, Request, Response, Status};

#[derive(Debug)]
pub(crate) struct DividerService {
    config: Config,
    term_channel: Arc<Mutex<Sender<()>>>,
    add_client: Arc<Mutex<Option<AdderClient<Channel>>>>,
    subtract_client: Arc<Mutex<Option<SubtractorClient<Channel>>>>,
    multiply_client: Arc<Mutex<Option<MultiplierClient<Channel>>>>,
}

impl DividerService {
    /// Create new DividerService - get whatever external service connections we can on boot
    /// The others can be initialized at request time (cold start problem - all micro services start roughly the same time but have
    /// inter dependencies and require a persistant TCP connection)
    pub(crate) async fn new(config: &Config, term_channel: Sender<()>) -> Self {
        Self {
            config: config.clone(),
            term_channel: Arc::new(Mutex::new(term_channel)),
            add_client: Arc::new(Mutex::new(build_adder_client(&config).await.ok())),
            subtract_client: Arc::new(Mutex::new(build_subtractor_client(&config).await.ok())),
            multiply_client: Arc::new(Mutex::new(build_multiplier_client(&config).await.ok())),
        }
    }

    /// Get current addition service connection or try again
    async fn get_add_client(&self) -> Result<AdderClient<Channel>, Error> {
        let mut ac = self.add_client.lock().await;

        if ac.is_none() {
            println!("Divider has no add client - retrying");
            let res = build_adder_client(&self.config).await.unwrap();
            println!("Multiplier divide client retry result {:?}", &res);
            *ac = Some(res);
        }

        ac.clone()
            .ok_or_else(|| Error::NoClientConnectionEstablished)
    }

    /// Get current subtraction service connection or try again
    async fn get_subtract_client(&self) -> Result<SubtractorClient<Channel>, Error> {
        let mut sc = self.subtract_client.lock().await;

        if sc.is_none() {
            println!("Divider has no subtract client - retrying");
            let res = build_subtractor_client(&self.config).await.unwrap();
            println!("Divider subtract client retry result {:?}", &res);
            *sc = Some(res);
        }

        sc.clone()
            .ok_or_else(|| Error::NoClientConnectionEstablished)
    }

    /// Get current multiplication service connection or try again
    async fn get_mutiply_client(&self) -> Result<MultiplierClient<Channel>, Error> {
        let mut mc: tokio::sync::MutexGuard<'_, Option<MultiplierClient<Channel>>> =
            self.multiply_client.lock().await;

        if mc.is_none() {
            println!("Divider has no multiply client - retrying");
            let res = build_multiplier_client(&self.config).await.unwrap();
            println!("Divider multiply client retry result {:?}", &res);
            *mc = Some(res);
        }

        mc.clone()
            .ok_or_else(|| Error::NoClientConnectionEstablished)
    }
}

#[async_trait]
impl MathASTEvaluator<Error> for DividerService {
    async fn add(&self, first: i32, second: i32) -> Result<i32, Error> {
        println!("Divider Delegate Add: {:?} + {:?}", first, second);
        let message = CalculationRequest {
            first_arg: serde_json::to_string(&MathAST::Value(first)).map_err(Error::SerdeJSON)?,
            second_arg: serde_json::to_string(&MathAST::Value(second)).map_err(Error::SerdeJSON)?,
        };

        let mut add_client = self.get_add_client().await?;

        let res = add_client
            .add(message)
            .await
            .map_err(Error::ExternalServiceStatus)?
            .into_inner();

        Ok(res.result)
    }

    async fn subtract(&self, first: i32, second: i32) -> Result<i32, Error> {
        println!("Divider Delegate Subtract: {:?} - {:?}", first, second);
        let message = CalculationRequest {
            first_arg: serde_json::to_string(&MathAST::Value(first)).map_err(Error::SerdeJSON)?,
            second_arg: serde_json::to_string(&MathAST::Value(second)).map_err(Error::SerdeJSON)?,
        };

        let mut subtract_client = self.get_subtract_client().await?;

        let res = subtract_client
            .subtract(message)
            .await
            .map_err(Error::ExternalServiceStatus)?
            .into_inner();

        Ok(res.result)
    }

    async fn multiply(&self, first: i32, second: i32) -> Result<i32, Error> {
        println!("Divider Delegate Multiply: {:?} * {:?}", first, second);
        let message = CalculationRequest {
            first_arg: serde_json::to_string(&MathAST::Value(first)).map_err(Error::SerdeJSON)?,
            second_arg: serde_json::to_string(&MathAST::Value(second)).map_err(Error::SerdeJSON)?,
        };

        let mut multiply_client = self.get_mutiply_client().await?;

        let res = multiply_client
            .multiply(message)
            .await
            .map_err(Error::ExternalServiceStatus)?
            .into_inner();

        Ok(res.result)
    }

    async fn divide(&self, first: i32, second: i32) -> Result<i32, Error> {
        println!("Divider Divide: {:?} / {:?}", first, second);
        Ok(first / second)
    }
}

#[tonic::async_trait]
impl Divider for DividerService {
    async fn divide(
        &self,
        request: Request<CalculationRequest>,
    ) -> Result<Response<CalculationResponse>, Status> {
        let inner = request.into_inner();

        let first: MathAST = serde_json::from_str(&inner.first_arg).map_err(|_| {
            Status::invalid_argument(format!("Invalid AST: {:#?}", &inner.first_arg))
        })?;
        let second: MathAST = serde_json::from_str(&inner.second_arg).map_err(|_| {
            Status::invalid_argument(format!("Invalid AST: {:#?}", &inner.second_arg))
        })?;

        let res = try_from_ast(self, MathAST::Divide(Box::new(first), Box::new(second))).await?;

        Ok(Response::new(TryInto::<CalculationResponse>::try_into(
            res,
        )?))
    }

    async fn term(&self, _: Request<Empty>) -> Result<Response<Empty>, Status> {
        let channel = self.term_channel.lock().await;
        let _ = channel.send(()).await;

        Ok(Response::new(Empty {}))
    }
}

/// See if we can get MathAST::Value(int32) from current AST - if not recurse and try again after running eval()
#[async_recursion]
async fn try_from_ast(service: &DividerService, ast: MathAST) -> Result<MathAST, Error> {
    if let MathAST::Value(_) = &ast {
        Ok(ast)
    } else {
        try_from_ast(service, service.eval(ast).await?).await
    }
}
