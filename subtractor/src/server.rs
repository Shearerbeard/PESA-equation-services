use std::sync::Arc;

use async_recursion::async_recursion;
use async_trait::async_trait;
use equation::{
    client::{build_adder_client, build_divider_client, build_multiplier_client},
    config::Config,
    parse::{MathAST, MathASTEvaluator},
    proto::equation::{
        adder_client::AdderClient, divider_client::DividerClient,
        multiplier_client::MultiplierClient, subtractor_server::Subtractor, CalculationRequest,
        CalculationResponse,
    },
    server::Error,
};
use tokio::sync::Mutex;
use tonic::{transport::Channel, Request, Response, Status};

#[derive(Debug)]
pub(crate) struct SubtractorService {
    config: Config,
    add_client: Arc<Mutex<Option<AdderClient<Channel>>>>,
    multiply_client: Arc<Mutex<Option<MultiplierClient<Channel>>>>,
    divide_client: Arc<Mutex<Option<DividerClient<Channel>>>>,
}

impl SubtractorService {
    /// Create new SubtractorService - get whatever external service connections we can on boot
    /// The others can be initialized at request time (cold start problem - all micro services start roughly the same time but have
    /// inter dependencies and require a persistant TCP connection)
    pub(crate) async fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
            add_client: Arc::new(Mutex::new(build_adder_client(&config).await.ok())),
            multiply_client: Arc::new(Mutex::new(build_multiplier_client(&config).await.ok())),
            divide_client: Arc::new(Mutex::new(build_divider_client(&config).await.ok())),
        }
    }

    /// Get current addition service connection or try again
    async fn get_add_client(&self) -> Result<AdderClient<Channel>, Error> {
        let mut ac = self.add_client.lock().await;

        if ac.is_none() {
            println!("Subractor has no add client - retrying");
            let res = build_adder_client(&self.config).await.unwrap();
            println!("Subtractor add client retry result {:?}", &res);
            *ac = Some(res);
        }

        ac.clone()
            .ok_or_else(|| Error::NoClientConnectionEstablished)
    }

    /// Get current multiplication service connection or try again
    async fn get_mutiply_client(&self) -> Result<MultiplierClient<Channel>, Error> {
        let mut mc: tokio::sync::MutexGuard<'_, Option<MultiplierClient<Channel>>> =
            self.multiply_client.lock().await;

        if mc.is_none() {
            println!("Subractor has no multiply client - retrying");
            let res = build_multiplier_client(&self.config).await.unwrap();
            println!("Subtractor multiply client retry result {:?}", &res);
            *mc = Some(res);
        }

        mc.clone()
            .ok_or_else(|| Error::NoClientConnectionEstablished)
    }

    /// Get current division service connection or try again
    async fn get_divider_client(&self) -> Result<DividerClient<Channel>, Error> {
        let mut ac: tokio::sync::MutexGuard<'_, Option<DividerClient<Channel>>> =
            self.divide_client.lock().await;

        if ac.is_none() {
            println!("Subractor has no divide client - retrying");
            let res = build_divider_client(&self.config).await.unwrap();
            println!("Subtractor divide client retry result {:?}", &res);
            *ac = Some(res);
        }

        ac.clone()
            .ok_or_else(|| Error::NoClientConnectionEstablished)
    }
}

#[async_trait]
impl MathASTEvaluator<Error> for SubtractorService {
    async fn add(&self, first: i32, second: i32) -> Result<i32, Error> {
        println!("Subtractor Delegate Add: {:?} + {:?}", first, second);
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
        println!("Subtractor Subtract: {:?} - {:?}", first, second);
        Ok(first - second)
    }
    async fn multiply(&self, first: i32, second: i32) -> Result<i32, Error> {
        println!("Subtractor Delegate Multiply: {:?} * {:?}", first, second);
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
        println!("Subtractor Delegate Divide: {:?} / {:?}", first, second);
        let message = CalculationRequest {
            first_arg: serde_json::to_string(&MathAST::Value(first)).map_err(Error::SerdeJSON)?,
            second_arg: serde_json::to_string(&MathAST::Value(second)).map_err(Error::SerdeJSON)?,
        };

        let mut divide_client = self.get_divider_client().await?;

        let res = divide_client
            .divide(message)
            .await
            .map_err(Error::ExternalServiceStatus)?
            .into_inner();

        Ok(res.result)
    }
}

#[tonic::async_trait]
impl Subtractor for SubtractorService {
    async fn subtract(
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

        let res = try_from_ast(self, MathAST::Subtract(Box::new(first), Box::new(second))).await?;

        Ok(Response::new(TryInto::<CalculationResponse>::try_into(
            res,
        )?))
    }
}

/// See if we can get MathAST::Value(int32) from current AST - if not recurse and try again after running eval()
#[async_recursion]
async fn try_from_ast(service: &SubtractorService, ast: MathAST) -> Result<MathAST, Error> {
    if let MathAST::Value(_) = &ast {
        Ok(ast)
    } else {
        try_from_ast(service, service.eval(ast).await?).await
    }
}
