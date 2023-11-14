use crate::error::Error;

pub(crate) fn memw(src: &[u8], dest: &mut [u8], addr: usize) -> Result<(), Error> {
    if addr + src.len() > dest.len() {
        return Err(Error::InvalidOpCode);
    }
    dest[addr..addr + src.len()].copy_from_slice(src);
    Ok(())
}

pub(crate) fn memr32(src: &[u8], addr: usize) -> Result<[u8; 4], Error> {
    if addr + 4 > src.len() {
        return Err(Error::InvalidOpCode);
    }
    let mut r = [0u8; 4];
    r[0..4].copy_from_slice(&src[addr..addr + 4]);
    Ok(r)
}

pub(crate) fn memr16(src: &[u8], addr: usize) -> Result<[u8; 2], Error> {
    if addr + 4 > src.len() {
        return Err(Error::InvalidOpCode);
    }
    let mut r = [0u8; 2];
    r[0..2].copy_from_slice(&src[addr..addr + 2]);
    Ok(r)
}

pub(crate) fn memr8(src: &[u8], addr: usize) -> Result<u8, Error> {
    if addr + 4 > src.len() {
        return Err(Error::InvalidOpCode);
    }
    Ok(src[addr])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memw() {
        let mut memory = [0u8; 1024];
        let data = "hello world!";
        memw(data.as_bytes(), &mut memory, 0x0).unwrap();
        assert_eq!(data.as_bytes(), &memory[0..data.len()])
    }

    #[test]
    fn test_memr32() {
        let mut memory = [0u8; 1024];
        let data = "hello_world!";
        memw(data.as_bytes(), &mut memory, 0x0).unwrap();
        let read = memr32(&memory, 0x0).unwrap();
        assert_eq!(&read, "hell".as_bytes());
    }

    #[test]
    fn test_memr16() {
        let mut memory = [0u8; 1024];
        let data = "hello_world!";
        memw(data.as_bytes(), &mut memory, 0x0).unwrap();
        let read = memr16(&memory, 0x0).unwrap();
        assert_eq!(&read, "he".as_bytes());
    }

    #[test]
    fn test_memr8() {
        let mut memory = [0u8; 1024];
        let data = "hello_world!";
        memw(data.as_bytes(), &mut memory, 0x0).unwrap();
        let read = memr8(&memory, 0x0).unwrap();
        assert_eq!(read, 'h' as u8);
    }
}
