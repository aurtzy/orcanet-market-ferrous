pub mod market {
    tonic::include_proto!("market"); // Path to proto file
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = stdin();
    let mut client = MarketClient::connect("http://127.0.0.1:50051")
        .await
        .unwrap();
    
    let mut user = String::new();
    print!("Enter a username: ");
    let _ = stdout().flush();
    scan.read_line(&mut user).unwrap();
    let user = user.trim();

    println!();
    print!("Enter a price: ");
    let _ = stdout().flush();
    let mut price = String::new();
    scan.read_line(&mut price).unwrap();
    let price: u32 = price.trim().parse().unwrap();

    let user = User {
        name: user.to_owned(),
        id: "1".to_owned(),
        port: 416320,
        ip: "localhost".to_owned(),
        price: price.into(),
    };
    println!();

    loop {
        let mut s = String::new();
        println!("--------------");
        println!("1. Register file");
        println!("2. Check holders");
        println!("3. Exit");
        
        print!("Enter your choice: ");
    
        let _ = stdout().flush();
        scan.read_line(&mut s).unwrap();
    
        let choice: u32 = s.trim().parse().unwrap();

        if choice == 3 {
            break;
        }

        let mut file_hash = String::new();
        print!("Enter file hash: ");
        let _ = stdout().flush();
        scan.read_line(&mut file_hash).unwrap();
        let file_hash = file_hash.trim();

        match choice {
            1 => {
                register_file(&mut client, file_hash, &user).await;
            }
            2 => {
                check_holders(&mut client, file_hash).await;
            }
            _ => {
                println!("Invalid choice");
            }
        }

        println!();
    }

    Ok(())
}

async fn register_file(client: &mut MarketClient<tonic::transport::Channel>, file_hash: &str, user: &User) -> Result<(), Box<dyn std::error::Error>> {
    let request = tonic::Request::new(RegisterFileRequest {
        file_hash: file_hash.to_owned(),
        user: Some(user.clone()),
    });

    client.register_file(request).await.unwrap();
    println!("File registered");
    
    Ok(())
}


async fn check_holders(client: &mut MarketClient<tonic::transport::Channel>, file_hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    let request = tonic::Request::new(CheckHoldersRequest {
        file_hash: file_hash.to_owned(),
    });

    let response = client.check_holders(request).await.unwrap();
    let holders = response.into_inner().holders;

    println!("Holders:");
    for holder in holders {
        println!("User {} is charging {}", holder.name, holder.price);
    }

    Ok(())
}
