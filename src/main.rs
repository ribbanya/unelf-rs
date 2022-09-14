use simple_logger::SimpleLogger;
use std::{
    fs::{
        self,
        File,
    },
    io::Write,
    time::Instant,
};
use log::{debug, LevelFilter};
use unwrap_elf::settings::Settings;
use object::{
    Object,
    ObjectSection,
    ObjectSymbol,
    ObjectSymbolTable,
    SymbolKind::Text,
};
use ppc750cl::{disasm_iter};


fn main() {
    SimpleLogger::new()
        .with_level(
            if cfg!(debug_assertions) {
                LevelFilter::Debug
            } else {
                LevelFilter::Error
            })
        .init()
        .unwrap();

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
    let elf_file_ref = elf_file.as_ref();
    let symbol_table = elf_file_ref.and_then(|o| o.symbol_table());
    let mut out_file = File::create({
        let exe_path = std::env::current_exe().unwrap();
        exe_path.parent().unwrap().join("out.s")
    }).unwrap();

    elf_file_ref
        .and(symbol_table)
        .as_ref()
        .map(move |t| t.symbols())
        .into_iter()
        .by_ref()
        .flatten()
        .filter(move |sym| sym.is_definition() && {
            if sym.size() > 0 {
                true
            } else {
                debug!("'{}' has no size", sym.name().unwrap_or(&sym.address().to_string()));
                false
            }
        } && sym.kind() == Text)
        .filter_map(move |sym| {
            let index = sym.section_index();
            let section = index
                .and_then(|i| elf_file_ref
                    .unwrap()
                    .section_by_index(i)
                    .ok());
            let address: u32 = sym.address().try_into().unwrap();

            section
                .and_then(|c| c.data_range(address as u64, sym.size()).ok())
                .flatten()
                .map(|c| disasm_iter(c, address))
                .map(|disasm| (sym, disasm))
        })
        .for_each(|(sym, disasm)| {
            if let Ok(name) = sym.name() {
                write!(out_file, "{name}:\n").unwrap();
                for ins in disasm {
                    let code = ins.code;
                    let address = ins.addr;
                    let simplified = ins.simplified();
                    let simplified_str = simplified.to_string();
                    write!(out_file, "/* {address:08X} {code:08X} */ {simplified_str}\n").unwrap();
                }
            }
        });


    println!("\nElapsed time: {:.2?}", before.elapsed());
}
