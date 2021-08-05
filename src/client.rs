// !Sample implementation of a gRPC Client for Clawflake, not meant to production!

pub mod clawflake {
  tonic::include_proto!("clawflake");
}

use clawflake::clawflake_client::ClawflakeClient;
use clawflake::IdRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "http://[::0]:50051";

    let mut client = ClawflakeClient::connect(addr).await?;

    println!("Client connected to {}", addr);
    let request = tonic::Request::new(IdRequest {});

    println!("Trying to get an ID");
    let response = client.get_id(request).await?;

    println!("Received: {:?}", response);

    Ok(())
}
