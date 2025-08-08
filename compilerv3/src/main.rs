mod pre_procesor;
mod env_get;
mod compile_step;
fn main() {
    let file_name = env_get::get_env();
    let file_contents = env_get::get_file(&file_name);
    let to_asm = pre_procesor::pre_procesor::pre_proces(&file_contents);
    let _ = compile_step::compile(&to_asm);
}
