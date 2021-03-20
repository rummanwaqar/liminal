use hello::say_client::SayClient;
use hello::SayRequest;
mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://[::1]:2000")
        .connect()
        .await?;

    let mut client = SayClient::new(channel);
    let request = tonic::Request::new(
        SayRequest {
            name: String::from("rumman"),
        }
    );
    let response = client.send(request).await?.into_inner();
    println!("RESPONSE = {:?}", response);
    Ok(())
}