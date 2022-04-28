use crate::CustomerServicesClient;
use tonic::transport::Channel;

#[derive(Debug, Clone)]
pub struct Env {
    pub debug: bool,
    pub grpc_customer_client: CustomerServicesClient<Channel>,
}

impl Env {
    pub fn new(debug: bool, grpc_customer_client: CustomerServicesClient<Channel>) -> Self {
        Self {
            debug,
            grpc_customer_client,
        }
    }
}
