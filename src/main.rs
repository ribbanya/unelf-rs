use std::fs;
use std::time::Instant;
use unwrap_elf::settings::Settings;
use object::{Object, ObjectSymbolTable};

fn main() {
    let settings = Settings::new();
    let bin_data = settings
        .ok()
        .map(|s| s.elf)
        .and_then(|e| e.path)
        .map(fs::read)
        .unwrap()
        .unwrap();
    let before = Instant::now();
    let elf_file = object::File::parse(&*bin_data).ok();
    let symbol_table = elf_file.as_ref().and_then(|o| o.symbol_table());
    if let Some(symbols) = symbol_table.as_ref().map(|t| t.symbols()) {
        for s in symbols {
            println!("{s:?}");
        }
    }
    println!("\nElapsed time: {:.2?}", before.elapsed());
}
