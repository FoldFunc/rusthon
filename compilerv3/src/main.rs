mod pre_procesor;
mod env_get;
fn main() {
    let file_name = env_get::get_env();
    let file_contents = env_get::get_file(&file_name);
    println!("File to pre proces: \n{}", file_contents);
    pre_procesor::pre_procesor::pre_proces(&file_contents);
}
