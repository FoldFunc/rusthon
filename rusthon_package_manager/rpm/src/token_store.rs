use std::error::Error;
use std::fs::{self, create_dir_all, File};
use std::io::Write;
use std::path::Path;
pub fn store_tokens(token: String)  -> std::io::Result<()> {
    let dir_path = Path::new("/tmp/rpm");
    create_dir_all(dir_path)?;
    let file_path = dir_path.join("token.txt");
    let mut file = File::create(&file_path)?;
    writeln!(file, "{}", token)?;
    println!("Token stored");
    Ok(())
}
pub fn get_tokens() -> String {
    let dir_path = Path::new("/tmp/rpm");
    let file_path = dir_path.join("token.txt");
    if file_path.exists() {
        let contents = fs::read_to_string(file_path)
            .expect("Not able to read file");
        return contents.to_string()
    } else {
        return "".to_string()
    }
}
pub fn remove_tokens() -> std::io::Result<()> {
    let dir_path = Path::new("/tmp/rpm");
    let file_path = dir_path.join("token.txt");
    std::fs::remove_file(file_path);

    Ok(())
}
