use market::{CheckHoldersRequest, HoldersResponse, RegisterFileRequest, User};

use market::market_client::MarketClient;
use std::io::{stdin, stdout, Write};

pub mod market {
    tonic::include_proto!("market");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = stdin();
    let mut client = MarketClient::connect("http://127.0.0.1:50051")
        .await
        .unwrap();

    let mut s = String::new();
    println!("Option: ");

    scan.read_line(&mut s)?;
    let choice: i32 = s.trim().parse()?;

    let request = tonic::Request::new(CheckHoldersRequest {
        file_hash: "foo".to_owned(),
    });

    let response = client.check_holders(request).await.unwrap();

    dbg!(response);

    Ok(())
}
