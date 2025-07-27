mod token_store;
mod register;
mod login;
mod input;
mod env_args;
use login::login::LoginResponse;
fn main() {
    let args = env_args::get_env_args();
    println!("Args: {:?}", args);
    match args[1].as_str() {
        "logout" => {
            let token_user = token_store::get_tokens();
            let valid_token = login::login::check_token(&token_user);
            let _logged_out = login::login::logout(&token_user);
        }
        "register" => {
            let email = input::input("Enter your email for register: ".to_string());
            let password = input::input("Enter your password for register: ".to_string());
            let _ = register::register::call_register_server(email, password);
        }
        "login" => {
            let token_user = token_store::get_tokens();
            let valid_token_user = login::login::check_token(&token_user);
            if !valid_token_user.unwrap() {
                println!("Token not found please login with your credentials.");
                let email = input::input("Enter your email for register: ".to_string());
                let password = input::input("Enter your password for register: ".to_string());
                let body = login::login::call_login_server(email, password).unwrap();
                println!("Token: {}", body.token);
                println!("Body: {}", body.message);
                let token = body.token;
                let store_token = token_store::store_tokens(token);
            }else {
                println!("Logged in...")
            }
        }
        _ => panic!("Invalid option in argument 1."),
    }
}
