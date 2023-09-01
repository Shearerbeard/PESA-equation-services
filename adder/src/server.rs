use std::sync::Arc;

use async_recursion::async_recursion;
use async_trait::async_trait;
use equation::{
    parse::{MathAST, MathASTEvaluator},
    proto::equation::{
        adder_server::Adder, divider_client::DividerClient, multiplier_client::MultiplierClient,
        subtractor_client::SubtractorClient, CalculationRequest, CalculationResponse,
    },
};
use tokio::sync::Mutex;
use tonic::{transport::Channel, Request, Response, Status};

use crate::error::Error;

#[derive(Debug)]
pub(crate) struct AdderService {
    subtract_client: Arc<Mutex<SubtractorClient<Channel>>>,
    multiply_client: Arc<Mutex<MultiplierClient<Channel>>>,
    divide_client: Arc<Mutex<DividerClient<Channel>>>,
}

// pub (crate) struct ExternalServices {

// }

impl AdderService {
    pub(crate) fn new(
        sub: SubtractorClient<Channel>,
        mult: MultiplierClient<Channel>,
        div: DividerClient<Channel>,
    ) -> Self {
        Self {
            subtract_client: Arc::new(Mutex::new(sub)),
            multiply_client: Arc::new(Mutex::new(mult)),
            divide_client: Arc::new(Mutex::new(div)),
        }
    }
}

#[async_trait]
impl MathASTEvaluator<Error> for AdderService {
    async fn add(&self, first: i32, second: i32) -> Result<i32, Error> {
        Ok(first + second)
    }
    async fn subtract(&self, first: i32, second: i32) -> Result<i32, Error> {
        let message = CalculationRequest {
            first_arg: serde_json::to_string(&MathAST::Value(first)).map_err(Error::SerdeJSON)?,
            second_arg: serde_json::to_string(&MathAST::Value(second)).map_err(Error::SerdeJSON)?,
        };

        let mut subtract_client = self.subtract_client.lock().await;

        let res = subtract_client
            .subtract(message)
            .await
            .map_err(Error::ExternalServiceStatus)?
            .into_inner();

        Ok(res.result)
    }
    async fn multiply(&self, first: i32, second: i32) -> Result<i32, Error> {
        let message = CalculationRequest {
            first_arg: serde_json::to_string(&MathAST::Value(first)).map_err(Error::SerdeJSON)?,
            second_arg: serde_json::to_string(&MathAST::Value(second)).map_err(Error::SerdeJSON)?,
        };

        let mut multiply_client = self.multiply_client.lock().await;

        let res = multiply_client
            .multiply(message)
            .await
            .map_err(Error::ExternalServiceStatus)?
            .into_inner();

        Ok(res.result)
    }
    async fn divide(&self, first: i32, second: i32) -> Result<i32, Error> {
        let message = CalculationRequest {
            first_arg: serde_json::to_string(&MathAST::Value(first)).map_err(Error::SerdeJSON)?,
            second_arg: serde_json::to_string(&MathAST::Value(second)).map_err(Error::SerdeJSON)?,
        };

        let mut divide_client = self.divide_client.lock().await;

        let res = divide_client
            .divide(message)
            .await
            .map_err(Error::ExternalServiceStatus)?
            .into_inner();

        Ok(res.result)
    }
}

#[tonic::async_trait]
impl Adder for AdderService {
    async fn add(
        &self,
        request: Request<CalculationRequest>,
    ) -> Result<Response<CalculationResponse>, Status> {
        let inner = request.into_inner();

        let first: MathAST = serde_json::from_str(&inner.first_arg).map_err(|_| {
            Status::invalid_argument(format!("Invalid AST: {:#?}", &inner.first_arg))
        })?;
        let second: MathAST = serde_json::from_str(&inner.first_arg).map_err(|_| {
            Status::invalid_argument(format!("Invalid AST: {:#?}", &inner.second_arg))
        })?;

        let res = try_from_ast(self, MathAST::Add(Box::new(first), Box::new(second))).await?;

        Ok(Response::new(TryInto::<CalculationResponse>::try_into(
            res,
        )?))
    }
}

#[async_recursion]
async fn try_from_ast(service: &AdderService, ast: MathAST) -> Result<MathAST, Error> {
    if let MathAST::Value(_) = &ast {
        Ok(ast)
    } else {
        try_from_ast(service, service.eval(ast).await?).await
    }
}
