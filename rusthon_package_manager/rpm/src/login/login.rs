use reqwest::blocking::Client;
use std::collections::HashMap;
use std::error::Error;
pub fn call_login_server(email: String, password: String) -> Result<String, Box<dyn Error>> {
    println!("Logging in...");
    let client = Client::new();
    let mut data = HashMap::new();
    data.insert("email", email);
    data.insert("password", password);
    let res = client.post("http://127.0.0.1:8080/api/login")
        .json(&data)
        .send();
    match res {
        Ok(resp) => {
            let body = resp.text()?;
            println!("Server response: {}", body);
            return Ok(body);
        }
        Err(err) => {
            eprintln!("Failed to send data: {}", err);
        }
    }
    Ok(" ".to_string())
}
