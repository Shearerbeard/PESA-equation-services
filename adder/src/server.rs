use equation::proto::equation::{adder_server::Adder, CalculationRequest, CalculationResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub(crate) struct AdderServer {}

#[tonic::async_trait]
impl Adder for AdderServer {
    async fn add(
        &self,
        request: Request<CalculationRequest>,
    ) -> Result<Response<CalculationResponse>, Status> {
        let inner = request.into_inner();

        Ok(Response::new(CalculationResponse {
            id: inner.id,
            result: inner.first_arg + inner.second_arg,
        }))
    }
}
