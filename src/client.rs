use std::error::Error;

use proto::calculator_client::CalculatorClient;
use tonic::client;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let server_url = "http://127.0.0.1:50051";//"http://[::1]:50051";
    let mut client = CalculatorClient::connect(server_url).await?;

    let req = proto::CalculationRequest {
        a: 45,
        b: 55
    };

    let server_request = tonic::Request::new(req);
    let response = client.add(server_request).await?;
    print!("Response received: {:?}", response.get_ref().result);
    Ok(())
}