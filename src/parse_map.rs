use std::iter::Map;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use crate::{MainError::{self, *}};
// symbol_re = re.compile(
// r"^  (?P<local_addr>[\da-f]{8}) (?P<size>[\da-f]{6})"
// r" (?P<global_addr>[\da-f]{8})(?:  (?P<alignment>\d))?"
// r" (?P<name>@?[\.\w]+)(?: \(entry of (?P<parent>@?[\.\w]+)\))?"
// r" \t(?P<filename>@?[\w \.]+)\.o $")

pub(crate) fn parse_map(map_text: &str) -> Result<(), MainError> {
    let symbol_re = Regex::new(r"").map_err(RegexError)?;
    for capture in symbol_re.captures_iter(&map_text) {}
    Ok(())
}