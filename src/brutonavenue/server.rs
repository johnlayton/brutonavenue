use tonic::{transport::Server, Request, Response, Status};

use bruton_avenue::brutonavenue_service_server::{BrutonavenueService, BrutonavenueServiceServer};
use bruton_avenue::{BrutonavenueResponse, BrutonavenueRequest};

pub mod bruton_avenue {
    tonic::include_proto!("brutonavenue");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl BrutonavenueService for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<BrutonavenueRequest>,
    ) -> Result<Response<BrutonavenueResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let inner = request.into_inner();
        let reply = bruton_avenue::BrutonavenueResponse {
            id: inner.id,
            message: inner.name
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:5051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(BrutonavenueServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}