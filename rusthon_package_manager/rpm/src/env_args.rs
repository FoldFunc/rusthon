use std::env;
pub fn get_env_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid amount of arguments.")
    }
    return args;
}


