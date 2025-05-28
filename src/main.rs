use proto::calculator_server::{Calculator, CalculatorServer};
use proto::audit_server::{Audit, AuditServer};
use tonic::metadata::MetadataValue;
use tonic::{transport::Server};
use tonic::{Request, Status};


fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let required_token: MetadataValue<_> = "Bearer some-secret-token".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(request_token) if request_token == required_token => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token present")),
    }
}

pub mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = 
    tonic::include_file_descriptor_set!("calculator_descriptor");
}

type State = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug, Default)]
struct AuditService {
    state:State,
}

#[tonic::async_trait]
impl Audit for AuditService {
   async fn get_request_count(
    &self,
    _request: tonic::Request<proto::GetCountRequest>,
   ) -> Result<tonic::Response<proto::CounterResponse>, tonic::Status> {
        
        let count = self.state.read().await;
        println!("inside counter service: {}", *count);
        let response = proto::CounterResponse {
            count: *count,
        };
        Ok(tonic::Response::new(response))
   }
}


#[derive(Debug, Default)]
struct CalculatorService {
    state:State,
}

impl CalculatorService {
    async fn increment(&self) -> () {
        let mut count = self.state.write().await;
        *count += 1;
        println!("Request count : {}", *count);
    }
}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self, 
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {

        println!("Got a request: {:?}", request);
        self.increment().await;
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

        self.increment().await;
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

    let initial_state = State::default();
    let calculator_service = CalculatorService {
        state: initial_state.clone(),
    };
    let audit_service = AuditService {
        state: initial_state.clone(),
    };

    let refl_service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
    .build()?;
    

    Server::builder()
    .add_service(CalculatorServer::new(calculator_service))
    .add_service(AuditServer::with_interceptor(audit_service, check_auth))
    // for more powerful middleware(interceptor above) use tower instead of tonic
    .add_service(refl_service)
    .serve(addr)
    .await?;

    Ok(())
    
}
