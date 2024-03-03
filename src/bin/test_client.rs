use market::{CheckHoldersRequest, HoldersResponse, RegisterFileRequest, User};
use market::market_client::MarketClient;
use tonic::Request;

pub mod market {
    tonic::include_proto!("market"); // Path to proto file
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MarketClient::connect("http://127.0.0.1:50051").await.unwrap();

    // Test 1: Create a User and register a file 
    let user = User {
        id: "12345".into(),
        name: "Shuai Mu".into(),
        ip: "123.456.789.0".into(),
        port: 8000,
        price: 100
    };
    // Create a Register File Request with the user and file hash to be sent to the server
    let register_file_request = RegisterFileRequest {
        user: Some(user),
        // Note: in practice we should use hashing function, but for testing I hardcode the string
        file_hash: "ratcoin.js-hash".into()
    };
    // Send the register file request to the server
    let register_response = client.register_file(Request::new(register_file_request)).await.unwrap();
    println!("Test 1 - RegisterFile Response: {:?}", register_response);
    println!("Test 1 - SUCCESS\n");



    // Test 2: Check the Holders of a Registered File
    let check_holders_request = CheckHoldersRequest {
        file_hash: "ratcoin.js-hash".into()
    };
    // Send check holders request to the server and await response
    let check_holders_response = client.check_holders(Request::new(check_holders_request)).await.unwrap().into_inner();
    println!("Test 2 - CheckHolders Response: {:?}", check_holders_response);
    assert!(
        check_holders_response.holders.iter().any(|u| u.id == "12345" && u.name == "Shuai Mu"),
        "Test 2 Failed: User Shuai Mu should be a holder.\n"
    );
    println!("Test 2 - SUCCESS\n");



    // Test 3: Check the Holders of a NON-Registered File
    let check_holders_request = CheckHoldersRequest {
        file_hash: "shuaimu.jpeg-hash".into()
    };
    // Send check holders request to the server and await response
    let check_holders_response = client.check_holders(Request::new(check_holders_request)).await.unwrap().into_inner();
    println!("Test 3 - CheckHolders Response: {:?}", check_holders_response);
    assert!(
        check_holders_response.holders.is_empty(),
        "Test 3 Failed: There should be no holders for an unregistered file.\n"
    );
    println!("Test 3 - SUCCESS\n");



    // Test 4: Create seperate user querying the same file
    let user = User {
        id: "98765".into(),
        name: "Shumai".into(),
        ip: "100.200.300.400".into(),
        port: 8001,
        price: 101
    };
    // Create Check Holders Request for a Registered File
    let check_holders_request = CheckHoldersRequest {
        file_hash: "ratcoin.js-hash".into()
    };
    // Send check holders request to the server and await response
    let check_holders_response = client.check_holders(Request::new(check_holders_request)).await.unwrap().into_inner();
    println!("Test 4 - CheckHolders Response: {:?}", check_holders_response);
    // Assert that the user with ID "12345" is indeed a holder of the file.
    assert!(
        check_holders_response.holders.iter().any(|u| u.id == "12345"),
        "Test 4 Failed: Expected to find user with ID '12345' as a holder of the file 'ratcoin.js-hash'\n"
    );
    println!("Test 4 - SUCCESS\n");



    // Test 5: Attempt to Register a File with No User
    let register_file_request = RegisterFileRequest {
        user: None, // Deliberately not providing a user
        file_hash: "test-5-file-hash".into(),
    };
    let register_response = client.register_file(Request::new(register_file_request)).await;
    println!("Test 5 - Register Response: {:?}", register_response);
    assert!(register_response.is_err(), "Test 5 Failed: Registration of a file without a user should not succeed.");
    println!("Test 5 - SUCCESS\n");



    // Test 6: Register a File and Verify the Holder
    let user_for_test_6 = User {
        id: "user6".into(),
        name: "Bob".into(),
        ip: "192.168.1.6".into(),
        port: 8002,
        price: 60,
    };
    let register_file_request = RegisterFileRequest {
        user: Some(user_for_test_6),
        file_hash: "file-hash-test-6".into(),
    };
    client.register_file(Request::new(register_file_request)).await.unwrap();
    // Checking holders of the file
    let check_holders_request = CheckHoldersRequest {
        file_hash: "file-hash-test-6".into(),
    };
    let check_holders_response = client.check_holders(Request::new(check_holders_request)).await.unwrap().into_inner();
    println!("Test 6 - Check Holders Response: {:?}", check_holders_response);
    assert!(
        check_holders_response.holders.iter().any(|u| u.id == "user6"),
        "Test 6 Failed: User Six should be a holder of the file."
    );
    println!("Test 6 - SUCCESS\n");



    // Test 7: Check for Holders of a File that Doesn't Exist
    let check_holders_request = CheckHoldersRequest {
        file_hash: "nonexistent-file-hash".into(),
    };
    let check_holders_response = client.check_holders(Request::new(check_holders_request)).await.unwrap().into_inner();
    println!("Test 7 - Check Holders Response: {:?}", check_holders_response);
    assert!(
        check_holders_response.holders.is_empty(),
        "Test 7 Failed: A nonexistent file should have no holders."
    );
    println!("Test 7 - SUCCESS\n");

    Ok(())
}
