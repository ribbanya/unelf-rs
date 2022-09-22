use std::iter::Map;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use crate::{ExeHasNoParent, MainError};

// symbol_re = re.compile(
// r"^  (?P<local_addr>[\da-f]{8}) (?P<size>[\da-f]{6})"
// r" (?P<global_addr>[\da-f]{8})(?:  (?P<alignment>\d))?"
// r" (?P<name>@?[\.\w]+)(?: \(entry of (?P<parent>@?[\.\w]+)\))?"
// r" \t(?P<filename>@?[\w \.]+)\.o $")

fn parse_map(map_text: String) -> Result<(), MainError> {
    lazy_static! {
        static ref SYMBOL_RE: Result<&'static Regex ,MainError> = Regex::new("").map_err(|_| ExeHasNoParent).as_ref();
    }

    for capture in SYMBOL_RE.unwrap().captures_iter(&map_text) {
        
    }
    Ok(())
}