extern crate openssl;

use std::fmt;
use openssl::rsa::Rsa;


/// Structure with information about user.
///
/// For the server, it is necessary to save a list of users 
/// that are connected to it and their public keys. It is necessary 
/// for the user to store their generated keys. 
pub struct User {
    name: String,
    public_key: Vec<u8>,
    private_key: Option<Vec<u8>>
}


/// Implement `Display` trait for a `User` structure.
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let public_key = std::str::from_utf8(&self.public_key).unwrap();

        write!(f, "username: {}\n{}", self.name, public_key)
    }
}


/// Implement methods for `User` structure.
impl User {
    pub fn new(name: String, public_key: Option<Vec<u8>>) -> User {
        if let Some(public_key) = public_key {
            User {
                name: name,
                public_key: public_key,
                private_key: None
            }
        }
        else {
            let rsa = Rsa::generate(2048).unwrap();

            User {
                name: name,
                public_key: rsa.public_key_to_pem().unwrap(),
                private_key: Some(rsa.private_key_to_pem().unwrap())
            }
        }
    }
}
