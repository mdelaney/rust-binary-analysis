use super::SectionHeader;

pub fn get_section_by_name<'a>(
    name: &str,
    sections: &'a [SectionHeader],
) -> Option<&'a SectionHeader> {
    for section in sections {
        if section.name_string == String::from(name) {
            return Some(section);
        }
    }
    return None;
}
