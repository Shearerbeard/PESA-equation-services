use equation::proto::equation::{
    CalculationRequest, CalculationResponse, divider_server::Divider,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub(crate) struct DividerServer {}

#[tonic::async_trait]
impl Divider for DividerServer {
    async fn divide(
        &self,
        request: Request<CalculationRequest>,
    ) -> Result<Response<CalculationResponse>, Status> {
        let inner = request.into_inner();

        Ok(Response::new(CalculationResponse {
            id: inner.id,
            result: inner.first_arg / inner.second_arg,
        }))
    }
}