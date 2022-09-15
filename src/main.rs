use simple_logger::SimpleLogger;
use std::{fs::{
    self,
    File,
}, io, io::Write, time::Instant};
use std::any::Any;
use std::borrow::Borrow;
use std::error::Error;
use config::ConfigError;
use log::{debug, LevelFilter};
use unwrap_elf::settings::Settings;
use object::{
    Object,
    ObjectSection,
    ObjectSymbol,
    ObjectSymbolTable,
    SymbolKind::Text,
    File as ObjectFile,
};
use ppc750cl::{disasm_iter};

fn init_logger() {
    SimpleLogger::new()
        .with_level(
            if cfg!(debug_assertions) {
                LevelFilter::Debug
            } else {
                LevelFilter::Error
            })
        .init()
        .unwrap();
}



fn get_elf_file() -> Result<(Vec<u8>, ObjectFile<'static>), String> {
    let result =
        Settings::new()
            .map_err(move |err| err.to_string())
            .map(move |settings| settings.elf)
            .and_then(move |elf| elf.path
                .ok_or("No elf path found".to_string()))
            .and_then(move |path| fs::read(path).map_err(|err| err.to_string()))
            .and_then(move |data| {
                ObjectFile::parse(&*data)
                    .map_err(|err| err.to_string())
                    .and_then(|elf_file| Ok((data, elf_file)))
            })
        /*
         */
        ;
    result

    // .map_err(|err| err.to_string())

    // let elf = settings.elf;
    // let path = elf.path.ok_or("No elf path found")?;
    // let data = fs::read(path).map_err(|e| e.to_string())?;
    // let elf_file = object::File::parse(&*data).map_err(|e| e.to_string())?;
    // Ok(elf_file)
    // ?.elf.path.map(fs::read).ok_or(|e| e.to_string())?
}

fn main() {
    init_logger();

    let before = Instant::now();

    let (data, elf_file) = get_elf_file().unwrap();
    let symbol_table = elf_file.symbol_table();
    let elf_file_ref = Some(elf_file).as_ref();
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
