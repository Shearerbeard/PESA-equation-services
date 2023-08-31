use equation::proto::equation::{
    subtractor_server::Subtractor, CalculationRequest, CalculationResponse,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub(crate) struct SubtractorServer {}

#[tonic::async_trait]
impl Subtractor for SubtractorServer {
    async fn subtract(
        &self,
        request: Request<CalculationRequest>,
    ) -> Result<Response<CalculationResponse>, Status> {
        let inner = request.into_inner();

        Ok(Response::new(CalculationResponse {
            id: inner.id,
            result: inner.first_arg - inner.second_arg,
        }))
    }
}
