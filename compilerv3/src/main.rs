mod pre_procesor;
mod post_processor;
mod env_get;
fn main() {
    let file_name = env_get::get_env();
    let file_contents = env_get::get_file(&file_name);
    let to_asm = pre_procesor::pre_procesor::pre_proces(&file_contents);
    let to_ir = post_processor::post_procesor::post_proces(&to_asm);
    let _ = post_processor::compile_step::compile(&to_ir);
}
