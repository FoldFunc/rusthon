mod file_helpers;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let path = file_helpers::take_command_line_args()?;
    println!("Path to file to compile: {}", path);
    let _ = file_helpers::check_valid_path(path.clone())?;
    let file_contents = file_helpers::give_file_content(path.clone())?;
    println!("file contents: \n{:?}", file_contents);
    Ok(())
}
