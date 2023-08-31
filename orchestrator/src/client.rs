use equation::{
    config::Config,
    proto::equation::{
        adder_client::AdderClient, divider_client::DividerClient,
        multiplier_client::MultiplierClient, subtractor_client::SubtractorClient,
    },
};
use tonic::transport::Channel;

const SCHEME: &str = "http://";

pub(crate) async fn build_adder_client(
    config: &Config,
) -> Result<AdderClient<Channel>, tonic::transport::Error> {
    AdderClient::connect(build_url(&config.adder_addr)).await
}

pub(crate) async fn build_subtractor_client(
    config: &Config,
) -> Result<SubtractorClient<Channel>, tonic::transport::Error> {
    SubtractorClient::connect(build_url(&config.subtractor_addr)).await
}

pub(crate) async fn build_multiplier_client(
    config: &Config,
) -> Result<MultiplierClient<Channel>, tonic::transport::Error> {
    MultiplierClient::connect(build_url(&config.multiplier_addr)).await
}

pub(crate) async fn build_divider_client(
    config: &Config,
) -> Result<DividerClient<Channel>, tonic::transport::Error> {
    DividerClient::connect(build_url(&config.divider_addr)).await
}

fn build_url(conn_str: &str) -> String {
    let mut url = SCHEME.to_string();
    url.push_str(conn_str);
    url
}
