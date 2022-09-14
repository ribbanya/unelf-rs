use simple_logger::SimpleLogger;
use std::{
    fs::{
        self,
        File
    },
    io::Write,
    time::Instant
};
use log::{debug, LevelFilter};
use unwrap_elf::settings::Settings;
use object::{Object, ObjectSection, ObjectSymbol, ObjectSymbolTable};
use object::SymbolKind::Text;
use ppc750cl::{disasm_iter, Opcode};


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
        .map(|t| t.symbols())
        .into_iter()
        .by_ref()
        .flatten()
        .filter(|s| s.is_definition() && s.size() > 0 && s.kind() == Text)
        .filter_map(|s| {
            let index = s.section_index();
            let section = index
                .and_then(|i| elf_file_ref
                    .unwrap()
                    .section_by_index(i)
                    .ok());
            let address: u32 = s.address().try_into().unwrap();

            section
                .and_then(|c| c.data_range(address as u64, s.size()).ok())
                .flatten()
                .map(|c| disasm_iter(c, address))
        })
        .flatten()
        .for_each(|ins| {
            let code = ins.code;
            let mnemonic = Opcode::detect(code).mnemonic();
            write!(out_file, "{code:08X} {mnemonic}\n").unwrap();
            debug!("{}", mnemonic);
        });


    println!("\nElapsed time: {:.2?}", before.elapsed());
}
