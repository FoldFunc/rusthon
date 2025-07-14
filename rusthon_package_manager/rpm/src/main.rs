mod register;
mod login;
mod input;
mod env_args;
fn main() {
    let args = env_args::get_env_args();
    println!("Args: {:?}", args);
    match args[1].as_str() {
        "register" => {
            let email = input::input("Enter your email for register: ".to_string());
            let password = input::input("Enter your password for register: ".to_string());
            let _ = register::register::call_register_server(email, password);
        }
        "login" => {
            let email = input::input("Enter your email for register: ".to_string());
            let password = input::input("Enter your password for register: ".to_string());
            let token = login::login::call_login_server(email, password);
        }
        _ => panic!("Invalid option in argument 1."),
    }
}
