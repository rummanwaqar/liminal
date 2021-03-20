use tonic::{transport::Server, Request, Response, Status};

use hello::say_server::{Say, SayServer};
use hello::{SayRequest, SayResponse};

mod hello;

// struct for our service
#[derive(Default)]
pub struct MySay {}

// implement rpc for service defined in proto
#[tonic::async_trait]
impl Say for MySay {
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        Ok(Response::new(SayResponse {
            message: format!("hello {}!", request.get_ref().name),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:2000".parse().unwrap();
    let say = MySay::default();
    println!("Server is listening on {}", addr);
    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}
