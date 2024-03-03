pub mod market {
    tonic::include_proto!("market"); // Path to proto file
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
