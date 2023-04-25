use std::env;
use clap::{Parser, Subcommand};

use authentication::auth_client::AuthClient;
use authentication::{SignInRequest, SignOutRequest, SignUpRequest};
use tonic::transport::Channel;
use tonic::{Request, Response};

use crate::authentication::{SignUpResponse, SignInResponse, SignOutResponse};

pub mod authentication {
    tonic::include_proto!("authentication");
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    SignIn {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    SignUp {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    SignOut {
        #[arg(short, long)]
        session_token: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // AUTH_SERVICE_IP can be set to your droplet's ip address once your app is deployed
    let auth_ip = env::var("AUTH_SERVICE_IP").unwrap_or("[::0]".to_owned());
    let mut client: AuthClient<Channel> = todo!(); // Create new `AuthClient` instance. Propagate any errors.

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::SignIn { username, password }) => {
            let request: Request<SignInRequest> = todo!(); // Create a new `SignInRequest`.
        
            // Make a sign in request. Propagate any errors. Convert Response<SignInResponse> into SignInResponse.
            let response: SignInResponse = todo!();
        
            println!("{:?}", response);
        }
        Some(Commands::SignUp { username, password }) => {
            let request: Request<SignUpRequest> = todo!(); // Create a new `SignUpRequest`.
        
            let response: Response<SignUpResponse> = todo!(); // Make a sign up request. Propagate any errors.
        
            println!("{:?}", response.into_inner());
        }
        Some(Commands::SignOut { session_token }) => {
            let request: Request<SignOutRequest> = todo!(); // Create a new `SignOutRequest`.
        
            let response: Response<SignOutResponse> = todo!(); // Make a sign out request. Propagate any errors.
        
            println!("{:?}", response.into_inner());
        }
        None => {}
    }

    Ok(())
}
