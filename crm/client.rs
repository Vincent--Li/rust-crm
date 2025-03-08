use anyhow::Result;
use crm::{user_service_client::UserServiceClient, CreateUserRequest};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = UserServiceClient::connect("http://localhost:50051").await?;

    let request = Request::new(CreateUserRequest {
        name: "John Doe".to_string(),
        email: "JohnDoe@gmail.com".to_string(),
    });

    let response = client.create_user(request).await?;

    println!("{:?}", response);

    Ok(())
}
