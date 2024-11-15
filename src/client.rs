use proto::calculator_client::CalculatorClient;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://[::1]:50051";
    let mut client = CalculatorClient::connect(url).await?;

    let req = proto::CalculatorRequest {a : 99, b: 101};
    let request= tonic::Request::new(req);

    let response = client.add(request).await?;

    println!("Client get response {:?}", response.get_ref().result);

    Ok(())
}