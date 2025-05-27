use proto::calculator_server::{Calculator, CalculatorServer};
use tonic::transport::Server;

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
    ) -> Result<tonic::Response<proto::CalculaionResponse>, tonic::Status> {

        print!("Got a request: {:?}", request);
        let input = request.get_ref();

        let response = proto::CalculaionResponse {
            result : input.a + input.b,
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
