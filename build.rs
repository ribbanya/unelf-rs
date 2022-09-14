use fs_extra::{
    copy_items,
    dir::CopyOptions};
use std::{
    {path::Path, env::var}};

fn main() {
    let options = CopyOptions { overwrite: true, ..Default::default() };
    let output_path = {
        let manifest_dir_string = var("CARGO_MANIFEST_DIR").unwrap();
        let build_type = var("PROFILE").unwrap();
        Path::new(&manifest_dir_string).join("target").join(build_type)
    };

    match copy_items(&vec!["config"], &output_path, &options) {
        Ok(_) => (),
        Err(error) => println!("cargo:warning=error copying 'config': {}", error)
    };
}