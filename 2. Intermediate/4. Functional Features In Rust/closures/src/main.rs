struct Credentials<T> where T: Fn(&str, &str) -> bool {
    username: String,
    password: String,
    validator: T
}

impl<T> Credentials<T> where T: Fn(&str, &str) -> bool {
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    let validator = |username: &str, password: &str| {
        !username.is_empty() && !password.is_empty()
    };
    let weak_password = "password123!".to_owned();
    // Fn - Immutably borrow variables in environment.
    // FnMut - Mutably borrow variables in environment. Can change environment.
    // FnOnce - Take ownership of variables in environment. Can only be called once.
    let validator2 = |username: &str, password: &str| {
        !username.is_empty() &&
        !password.is_empty() &&
        password.len() > 8 &&
        password.contains(['!', '@', '#', '$', '%', '^', '&', '*']) &&
        password != weak_password
    };
    println!("{weak_password}");
    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password123!".to_owned(),
        validator: validator2
    };
    println!("{}", creds.is_valid());
    let password_validator = get_password_validator(8, true);
    let default_creds = get_default_creds(password_validator);
}

fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}

fn get_default_creds<T>(f: T) -> Credentials<T> where T: Fn(&str, &str) -> bool {
    Credentials {
        username: "guest".to_owned(),
        password: "password123!".to_owned(),
        validator: f
    }
}

fn get_password_validator(min_len: usize, special_char: bool) 
    -> Box<dyn Fn(&str, &str) -> bool> {
    if special_char {
        Box::new(move |_: &str, password: &str| {
            !password.len() >= min_len &&
            password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
        })
    } else {
        Box::new(move |_: &str, password: &str| !password.len() >= min_len)
    }
}