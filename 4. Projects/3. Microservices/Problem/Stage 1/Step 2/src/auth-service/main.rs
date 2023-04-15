use std::sync::Mutex;

mod auth;
mod sessions;
mod users;

use auth::*;
use sessions::{SessionsImpl, Sessions};
use users::{UsersImpl, Users};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces. This is needed for Docker to work, which we will add later on.
    // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
    // Port 50051 is the recommended gRPC port.
    let addr = "[::0]:50051".parse()?;

    let users_service: Box<Mutex<dyn Users + Send + Sync + 'static>> = todo!(); // Create user service instance
    let sessions_service: Box<Mutex<dyn Sessions + Send + Sync + 'static>> = todo!(); //Create session service instance

    let auth_service = AuthService::new(users_service, sessions_service);

    // Instantiate gRPC server
    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}
