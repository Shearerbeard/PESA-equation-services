use std::sync::Arc;

use async_recursion::async_recursion;
use async_trait::async_trait;
use equation::{
    client::{build_divider_client, build_multiplier_client, build_subtractor_client},
    config::Config,
    parse::{MathAST, MathASTEvaluator},
    proto::equation::{
        adder_server::Adder, divider_client::DividerClient, multiplier_client::MultiplierClient,
        subtractor_client::SubtractorClient, CalculationRequest, CalculationResponse,
    },
    server::Error,
};
use tokio::sync::Mutex;
use tonic::{transport::Channel, Request, Response, Status};

#[derive(Debug)]
pub(crate) struct AdderService {
    config: Config,
    subtract_client: Arc<Mutex<Option<SubtractorClient<Channel>>>>,
    multiply_client: Arc<Mutex<Option<MultiplierClient<Channel>>>>,
    divide_client: Arc<Mutex<Option<DividerClient<Channel>>>>,
}

impl AdderService {
    /// Create new AdderService - get whatever external service connections we can on boot
    /// The others can be initialized at request time (cold start problem - all micro services start roughly the same time but have
    /// inter dependencies and require a persistant TCP connection)
    pub(crate) async fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
            subtract_client: Arc::new(Mutex::new(build_subtractor_client(&config).await.ok())),
            multiply_client: Arc::new(Mutex::new(build_multiplier_client(&config).await.ok())),
            divide_client: Arc::new(Mutex::new(build_divider_client(&config).await.ok())),
        }
    }

    /// Get current subtraction service connection or try again
    async fn get_subtract_client(&self) -> Result<SubtractorClient<Channel>, Error> {
        let mut sc = self.subtract_client.lock().await;

        if sc.is_none() {
            println!("Adder has no subtract client - retrying");
            let res = build_subtractor_client(&self.config).await.unwrap();
            println!("Adder subtract client retry result {:?}", &res);
            *sc = Some(res);
        }

        sc.clone()
            .ok_or_else(|| Error::NoClientConnectionEstablished)
    }

    /// Get current multiplication service connection or try again
    async fn get_mutiply_client(&self) -> Result<MultiplierClient<Channel>, Error> {
        let mut mc = self.multiply_client.lock().await;

        if mc.is_none() {
            println!("Adder has no multiply client - retrying");
            let res = build_multiplier_client(&self.config).await.unwrap();
            println!("Adder multiply client retry result {:?}", &res);
            *mc = Some(res);
        }

        mc.clone()
            .ok_or_else(|| Error::NoClientConnectionEstablished)
    }

    /// Get current division service connection or try again
    async fn get_divider_client(&self) -> Result<DividerClient<Channel>, Error> {
        let mut dc = self.divide_client.lock().await;

        if dc.is_none() {
            println!("Adder has no multiply divide client- retrying");
            let res = build_divider_client(&self.config).await.unwrap();
            println!("Adder divide client retry result {:?}", &res);
            *dc = Some(res);
        }

        dc.clone()
            .ok_or_else(|| Error::NoClientConnectionEstablished)
    }
}

#[async_trait]
impl MathASTEvaluator<Error> for AdderService {
    async fn add(&self, first: i32, second: i32) -> Result<i32, Error> {
        println!("Adder Add: {:?} + {:?}", first, second);
        Ok(first + second)
    }
    async fn subtract(&self, first: i32, second: i32) -> Result<i32, Error> {
        println!("Adder Delegate Subtract: {:?} - {:?}", first, second);
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
        println!("Adder Delegate Multiply: {:?} * {:?}", first, second);
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
        println!("Adder Delegate Divide: {:?} / {:?}", first, second);
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
impl Adder for AdderService {
    async fn add(
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

        let res = try_from_ast(self, MathAST::Add(Box::new(first), Box::new(second))).await?;

        Ok(Response::new(TryInto::<CalculationResponse>::try_into(
            res,
        )?))
    }
}

/// See if we can get MathAST::Value(int32) from current AST - if not recurse and try again after running eval()
#[async_recursion]
async fn try_from_ast(service: &AdderService, ast: MathAST) -> Result<MathAST, Error> {
    if let MathAST::Value(_) = &ast {
        Ok(ast)
    } else {
        try_from_ast(service, service.eval(ast).await?).await
    }
}
