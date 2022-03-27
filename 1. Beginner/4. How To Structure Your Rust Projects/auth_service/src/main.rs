use auth_service::{Credentials};

fn main() {
    let creds = Credentials {
        username: "letsgetrusty".to_owned(),
        password: "password123".to_owned(),
    };

    auth_service::authenticate(creds);
}