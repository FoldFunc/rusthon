use std::env;
use std::fs;
use std::path::Path;
pub fn get_file(file_path: &String) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Couldn't read from a file");
    return contents;
}
pub fn get_env() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid amount of arguments, correct usage: ./compilerv3 <file_path>");
    }
    println!("args[1]: {}", args[1]);
    let mut is: bool = true;
    is = Path::new(&args[1]).exists();
    if !is {
        panic!("No such file: {}", args[1]);
    }else {
        return args[1].clone();
    }
}
