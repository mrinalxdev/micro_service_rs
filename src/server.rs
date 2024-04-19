use tonic::{transport::Server, Request, Response, Status};

use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BTCPaymentRequest, BTCPaymentResponse};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct BitcoinService {}

#[tonic::async_trait]
impl Bitcoin for BitcoinService {
    async fn send_payment(
        &self,
        request: Request<BTCPaymentRequest>,
    ) -> Result<Response<BTCPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = BTCPaymentRequest {
            successful: true,
            message: format!("Sent {}BTC to {}." req.amount, req.to_addr).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse();
    let btc_service = BitcoinService::default();

    Server::builder()
        .add_service(BitcoinService::new(btc_service))
        .serve(addr)
        .await?;
    Ok(())
}
