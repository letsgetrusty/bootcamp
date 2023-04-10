use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use rand_core::OsRng;
use uuid::Uuid;

use std::collections::HashMap;

pub trait Users {
    fn create_user(&mut self, username: String, password: String) -> Result<(), String>;
    fn get_user_uuid(&self, username: String, password: String) -> Option<String>;
    fn delete_user(&mut self, user_uuid: String);
}

#[derive(Clone)]
pub struct User {
    user_uuid: String,
    username: String,
    password: String,
}

#[derive(Default)]
pub struct UsersImpl {
    uuid_to_user: HashMap<String, User>,
    username_to_user: HashMap<String, User>,
}

impl Users for UsersImpl {
    fn create_user(&mut self, username: String, password: String) -> Result<(), String> {
        if self.username_to_user.contains_key(&username) {
            return Err("Unable to create user. Username already exists.".to_owned());
        }

        let salt = SaltString::generate(&mut OsRng);

        let hashed_password = Pbkdf2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Failed to hash password.\n{e:?}"))?
            .to_string();

        let user = User {
            user_uuid: Uuid::new_v4().to_string(),
            username: username.clone(),
            password: hashed_password,
        };


        self.username_to_user.insert(username, user.clone());
        self.uuid_to_user.insert(user.user_uuid.clone(), user);

        Ok(())
    }

    fn get_user_uuid(&self, username: String, password: String) -> Option<String> {
        let user = self.username_to_user.get(&username)?;

        let hashed_password = user.password.clone();
        let parsed_hash = PasswordHash::new(&hashed_password).ok()?;

        let result = Pbkdf2.verify_password(password.as_bytes(), &parsed_hash);

        if user.username == username && result.is_ok() {
            return Some(user.user_uuid.clone());
        }

        None
    }

    fn delete_user(&mut self, user_uuid: String) {
        if let Some(user) = self.uuid_to_user.get(&user_uuid) {
            let user_uuid = user.user_uuid.clone();
            self.username_to_user.remove(&user.username);
            self.uuid_to_user.remove(&user_uuid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_user() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        assert_eq!(user_service.uuid_to_user.len(), 1);
        assert_eq!(user_service.username_to_user.len(), 1);
    }

    #[test]
    fn should_fail_creating_user_with_existing_username() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        let result = user_service.create_user("username".to_owned(), "password".to_owned());

        assert!(result.is_err());
    }

    #[test]
    fn should_retrieve_user_uuid() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        assert!(user_service
            .get_user_uuid("username".to_owned(), "password".to_owned())
            .is_some());
    }

    #[test]
    fn should_fail_to_retrieve_user_uuid_with_incorrect_password() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        assert!(user_service
            .get_user_uuid("username".to_owned(), "incorrect password".to_owned())
            .is_none());
    }

    #[test]
    fn should_delete_user() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        let user_uuid = user_service
            .get_user_uuid("username".to_owned(), "password".to_owned())
            .unwrap();

        user_service.delete_user(user_uuid);

        assert_eq!(user_service.uuid_to_user.len(), 0);
        assert_eq!(user_service.username_to_user.len(), 0);
    }
}
