#![feature(rustc_private)]
fn main() {
  env_logger::init();
  rustc_plugin::cli_main(salt_ide::plugin::print_result::SaltPlugin);
}