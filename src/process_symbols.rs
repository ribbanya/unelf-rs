use std::{fs::File as FsFile, io::Write};
use log::warn;
use object::{
    Object, ObjectSection, ObjectSymbol, ObjectSymbolTable,
    File as ObjectFile,
    Symbol, SymbolKind::Text,
};
use ppc750cl::disasm_iter;
use crate::main_result::{MainError::*, MainResult};

pub(crate) fn process_symbols(elf_data: Vec<u8>, mut out: FsFile) -> MainResult {
    let elf = ObjectFile::parse(&*elf_data).map_err(ElfError)?;
    let symbol_table = elf.symbol_table().ok_or(MissingSymbolTable)?;
    let symbols = symbol_table.symbols().filter(filter_symbol);

    for symbol in symbols {
        process_symbol(symbol, &elf, &mut out)?;
    }

    Ok(())
}

fn filter_symbol(symbol: &Symbol) -> bool {
    if !symbol.is_definition() { return false; }
    if symbol.kind() != Text { return false; };

    if symbol.size() == 0 {
        warn!("{} has no size", {
            if let Ok(name) = symbol.name() {
                format!("'{name}'")
            } else {
                format!("{:X}", symbol.address())
            }
        });

        return false;
    }

    return true;
}

fn process_symbol(symbol: Symbol, elf: &ObjectFile, out: &mut dyn Write) -> MainResult {
    let address64 = symbol.address();

    let address32: u32 = match address64.try_into() {
        Ok(value) => value,
        Err(err) => {
            warn!("Couldn't convert {address64:X} to u32 ({err})");
            return Ok(());
        }
    };

    let index = if let Some(value) = symbol.section_index() { value } else {
        warn!("Couldn't get section index for symbol @{address32:08X}");
        return Ok(());
    };

    let section = elf.section_by_index(index).map_err(ElfError)?;

    let range = {
        let range_opt = section.data_range(address64, symbol.size()).map_err(ElfError)?;

        if let Some(value) = range_opt { value } else {
            warn!("Section {} is empty", {
                    if let Ok(name) = section.name() {
                        format!("'{}'", name)
                    } else {
                        format!("@{:X}", section.address())
                    }
                });
            return Ok(());
        }
    };

    let name = symbol.name().map_err(ElfError)?;
    let disasm = disasm_iter(range, address32);

    write!(out, "\n{name}:\n").map_err(FileError)?;

    for ins in disasm {
        let code = ins.code;
        let address = ins.addr;
        let simplified = ins.simplified();
        let simplified_str = simplified.to_string();
        write!(out, "/* {address:08X} {code:08X} */ {simplified_str}\n").map_err(FileError)?;
    }

    Ok(())
}
