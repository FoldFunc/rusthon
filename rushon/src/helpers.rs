use std::fs;
use std::path::Path;
use std::error::Error;
pub fn command_line_args() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("Invalid amount of command line arguments".into());
    }
    Ok(args[1].clone())
}
pub fn valid_path(path: &String) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    if path.exists() {
        return Ok(());
    } else {
        return Err("Invalid path to file.".into());
    }
}
pub fn file_context(path: &String) -> Result<String, Box<dyn Error>> {
    fs::read_to_string(path)
        .map_err(|e| format!("Error ocured when looking at file: {}", e).into())
}
