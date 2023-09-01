use tokio::{signal, sync::mpsc::Sender};
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

pub async fn wait_for_ctrl_c(tx: Sender<()>) {
    let _ = signal::ctrl_c().await;
    println!("SIGTERM received: shutting down");
    let _ = tx.send(());
}
