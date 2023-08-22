use tonic::{transport::Server, Request, Response, Status};

use payments::l1_payments_server::{L1Payments, L1PaymentsServer};
use payments::{L1PaymentRequest, L1PaymentResponse};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct L1Service {}

#[tonic::async_trait]
impl L1Payments for L1Service {
    async fn send_payment(
        &self,
        request: Request<L1PaymentRequest>,
    ) -> Result<Response<L1PaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = L1PaymentResponse {
            successful: true,
            message: format!("Sent {} L1Coin to {}.", req.amount, req.to_addr).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let btc_service = L1Service::default();

    Server::builder()
        .add_service(L1PaymentsServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}