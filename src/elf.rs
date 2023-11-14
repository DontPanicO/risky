use elf::{endian::LittleEndian, ElfBytes, ParseError};

pub(crate) fn load_elf_le(data: &[u8]) -> Result<ElfBytes<LittleEndian>, ParseError> {
    ElfBytes::<LittleEndian>::minimal_parse(data)
}
