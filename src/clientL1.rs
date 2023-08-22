use payments::l1_payments_client::{L1PaymentsClient};
use payments::L1PaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = L1PaymentsClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(
        L1PaymentRequest {
            from_addr: "0xLATTE".to_owned(),
            to_addr: "0xCAFE".to_owned(),
            amount: 88
        }
    );

    let response = client.send_payment(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}