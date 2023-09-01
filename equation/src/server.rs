use tonic::Status;

#[derive(Debug)]
pub enum Error {
    SerdeJSON(serde_json::Error),
    ExternalServiceStatus(Status),
    NoClientConnectionEstablished,
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        Status::internal(format!("Adder Service Error: {:#?}", value))
    }
}
