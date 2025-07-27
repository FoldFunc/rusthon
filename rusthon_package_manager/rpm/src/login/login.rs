use crate::token_store::remove_tokens;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::error::Error;
use serde::Deserialize;
#[derive(Debug, Deserialize)] // <- This should be Deserialize, not Serialize
pub struct LoginResponse {
    pub message: String,
    pub token: String,
}
#[derive(Debug, Deserialize)] // <- This should be Deserialize, not Serialize
pub struct LogoutResponse{
    pub message: String,
}
pub fn logout(token: &String) -> Result<LogoutResponse, Box<dyn Error>> {
    println!("Logging out...");
    let _ = remove_tokens();
    let client = Client::new();
    let mut data = HashMap::new();
    data.insert("token", token);
    let res = client
        .post("http://127.0.0.1:8080/api/logout")
        .json(&data)
        .send();
    match res {
        Ok(resp) => {
            let body = resp.json::<LogoutResponse>()?;
            Ok(body)
        }
        Err(e) => {
            eprintln!("Error while logging out: {}", e);
            Err(Box::new(e))
        }
    }
}
pub fn call_login_server(email: String, password: String) -> Result<LoginResponse, Box<dyn Error>> {
    println!("Logging in...");
    let client = Client::new();
    let mut data = HashMap::new();
    data.insert("email", email);
    data.insert("password", password);

    let res = client
        .post("http://127.0.0.1:8080/api/login")
        .json(&data)
        .send();

    match res {
        Ok(resp) => {
            let body = resp.json::<LoginResponse>()?;
            Ok(body)
        }
        Err(err) => {
            eprintln!("Failed to send data: {}", err);
            Err(Box::new(err))
        }
    }
}

pub fn check_token(token: &String) -> Result<bool, Box<dyn Error>> {
    println!("Checking if valid login token");

    let client = Client::new();
    let mut data = HashMap::new();
    data.insert("token", token);

    let res = client
        .post("http://127.0.0.1:8080/api/valid_token")
        .json(&data)
        .send()?;

    match res.status() {
        reqwest::StatusCode::OK => {
            println!("Good return from server");
            Ok(true)
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Unauthorized token");
            Ok(false)
        }
        status => {
            eprintln!("Unexpected server response: {}", status);
            Ok(false)
        }
    }
}
