use proto::calculator_server::{Calculator, CalculatorServer};
use tonic::{transport::Server, Request};

pub mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = 
    tonic::include_file_descriptor_set!("calculator_descriptor");
}

#[derive(Debug, Default)]
struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self, 
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {

        print!("Got a request: {:?}", request);
        let input = request.get_ref();

        let response = proto::CalculationResponse {
            result : input.a + input.b,
        };

        Ok(tonic::Response::new(response))
    }

    async fn divide(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {

        let inner_request = request.get_ref();
        if inner_request.b == 0 {
            return Err(tonic::Status::invalid_argument("Denominator cannot be zero"))
        }
        let response = proto::CalculationResponse {
            result: inner_request.a/inner_request.b,
        };
        Ok(tonic::Response::new(response))
    }
}

#[tokio:: main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let addr = "127.0.0.1:50051".parse()?;

    let calculator_service = CalculatorService::default();

    let refl_service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
    .build()?;

    Server::builder()
    .add_service(CalculatorServer::new(calculator_service))
    .add_service(refl_service)
    .serve(addr)
    .await?;

    Ok(())
    
}
