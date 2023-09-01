use equation::proto::equation::{divider_server::Divider, CalculationRequest, CalculationResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub(crate) struct DividerService {}

#[tonic::async_trait]
impl Divider for DividerService {
    async fn divide(
        &self,
        request: Request<CalculationRequest>,
    ) -> Result<Response<CalculationResponse>, Status> {
        let inner = request.into_inner();

        Ok(Response::new(CalculationResponse {
            result: inner.first_arg / inner.second_arg,
        }))
    }
}
