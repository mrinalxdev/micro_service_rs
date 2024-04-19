use payments::bitcoin_client::BitcoinClient;
use payments::BTCPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    //New Btc payments request
    let request = tonic::Request::new(
        BTCPaymentRequest{
            from_addr: "123456".to_owned(),
            to_addr: "6544321".to_owned(),
            amount : 22,
        }
    );

    let response = client.send_payment(request).await?;

    println!("RESPONSE ={:?}", response);
    Ok(())
}
