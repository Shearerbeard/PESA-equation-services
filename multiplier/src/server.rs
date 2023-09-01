use equation::proto::equation::{
    multiplier_server::Multiplier, CalculationRequest, CalculationResponse,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub(crate) struct MultiplierService {}

#[tonic::async_trait]
impl Multiplier for MultiplierService {
    async fn multiply(
        &self,
        request: Request<CalculationRequest>,
    ) -> Result<Response<CalculationResponse>, Status> {
        let inner = request.into_inner();

        Ok(Response::new(CalculationResponse {
            result: inner.first_arg * inner.second_arg,
        }))
    }
}
