#![allow(non_snake_case)]
use marine_rs_sdk::marine;

mod auth;

pub fn main() {}

#[marine]
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
}

#[marine]
pub fn auth_greeting(name: String) -> String {
    if !auth::is_owner() {
        panic!("You are not the owner of the service");
    }
    format!("Hi, {}", name)
}

#[marine]
pub fn panicked() {
    panic!("marine service panic!")
}
