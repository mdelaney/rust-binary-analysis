mod enums;
mod section_header;
mod utils;

use super::symbol::Symbol;
use super::utils::get_null_terminated_string_from_vec;
use crate::elf::elf_header::ELFHeader;
pub use section_header::SectionHeader;

fn _get_symbols(
    data: &[u8],
    table_section_name: &str,
    string_table_name: &str,
    section_headers: &[SectionHeader],
    elf_header: &ELFHeader,
) -> Result<Vec<Symbol>, &'static str> {
    let result: Vec<Symbol> = vec![];
    let symbol_table = match utils::get_section_by_name(table_section_name, section_headers) {
        Some(v) => v,
        None => return Ok(result),
    };
    let string_table = match utils::get_section_by_name(string_table_name, section_headers) {
        Some(v) => v,
        None => return Err("No string table section for the symbol section."),
    };

    let symtab_data = symbol_table.get_data(&data);
    let strings = string_table.get_data(&data);

    let mut result = Symbol::parse_from_symbol_table(symtab_data, elf_header);
    for symbol in &mut result {
        symbol.name_string = get_null_terminated_string_from_vec(strings, symbol.name as usize);
    }

    Ok(result)
}

// parses the dynamic symbol table if it exists
// making the assumption (should really verify) that the if ".dynsym" exists then
// ".dynstr" should also.
pub fn get_dynamic_symbols(
    data: &[u8],
    section_headers: &[SectionHeader],
    elf_header: &ELFHeader,
) -> Result<Vec<Symbol>, &'static str> {
    _get_symbols(data, ".dynsym", ".dynstr", section_headers, elf_header)
}

pub fn get_symbols(
    data: &[u8],
    section_headers: &[SectionHeader],
    elf_header: &ELFHeader,
) -> Result<Vec<Symbol>, &'static str> {
    _get_symbols(data, ".symtab", ".strtab", section_headers, elf_header)
}
