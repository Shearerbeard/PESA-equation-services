use equation::{
    config::Config,
    proto::equation::{
        adder_client::AdderClient, divider_client::DividerClient,
        multiplier_client::MultiplierClient, subtractor_client::SubtractorClient,
    },
};
use tonic::transport::Channel;

pub(crate) async fn build_adder_client(
    config: &Config,
) -> Result<AdderClient<Channel>, tonic::transport::Error> {
    AdderClient::connect(config.adder_addr.clone()).await
}

pub(crate) async fn build_subtractor_client(
    config: &Config,
) -> Result<SubtractorClient<Channel>, tonic::transport::Error> {
    SubtractorClient::connect(config.subtractor_addr.clone()).await
}

pub(crate) async fn build_multiplier_client(
    config: &Config,
) -> Result<MultiplierClient<Channel>, tonic::transport::Error> {
    MultiplierClient::connect(config.multiplier_addr.clone()).await
}

pub(crate) async fn build_divider_client(
    config: &Config,
) -> Result<DividerClient<Channel>, tonic::transport::Error> {
    DividerClient::connect(config.divider_addr.clone()).await
}
