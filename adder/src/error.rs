use tonic::Status;

#[derive(Debug)]
pub(crate) enum Error {
    SerdeJSON(serde_json::Error),
    ExternalServiceStatus(Status),
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        Status::internal(format!("Adder Service Error: {:#?}", value))
    }
}
