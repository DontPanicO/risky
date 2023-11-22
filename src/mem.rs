use crate::error::Error;

#[allow(clippy::missing_safety_doc)]
pub unsafe trait Pod: Copy {}

macro_rules! impl_pod {
    ($($t:ident($base:ty) -> $nname:ident;)*) => {
        $(
            #[repr(transparent)]
            #[derive(Clone, Copy)]
            pub struct $t($base);

            impl $t {
                #[inline(always)]
                pub const fn new(value: $base) -> Self {
                    Self(value.to_le())
                }

                #[inline(always)]
                pub const fn $nname(&self) -> $base {
                    <$base>::from_le(self.0)
                }
            }

            impl From<$base> for $t {
                #[inline(always)]
                fn from(value: $base) -> Self {
                    Self::new(value)
                }
            }

            impl From<$t> for $base {
                #[inline(always)]
                fn from(value: $t) -> Self {
                    value.$nname()
                }
            }

            unsafe impl Pod for $t {}
        )*
    };
}

unsafe impl Pod for u8 {}
unsafe impl Pod for i8 {}
impl_pod! {
    I16(i16) -> as_i16;
    U16(u16) -> as_u16;
    I32(i32) -> as_i32;
    U32(u32) -> as_u32;
    I64(i64) -> as_i64;
    U64(u64) -> as_u64;
}
unsafe impl<T: Pod, const N: usize> Pod for [T; N] {}

pub fn read<T: Pod>(src: &[u8], addr: usize) -> Result<T, Error> {
    Ok(unsafe {
        core::ptr::read_unaligned(
            src.get(addr..)
                .and_then(|src| src.get(..core::mem::size_of::<T>()))
                .ok_or(Error::InvalidOpCode)?
                .as_ptr()
                .cast(),
        )
    })
}

pub fn write<T: Pod>(src: &T, dest: &mut [u8], addr: usize) -> Result<(), Error> {
    unsafe {
        core::ptr::write_unaligned(
            dest.get_mut(addr..)
                .and_then(|dst| dst.get_mut(..core::mem::size_of::<T>()))
                .ok_or(Error::InvalidOpCode)?
                .as_mut_ptr()
                .cast(),
            *src,
        )
    };
    Ok(())
}

pub fn memw(src: &[u8], dest: &mut [u8], addr: usize) -> Result<(), Error> {
    let dest = dest
        .get_mut(addr..)
        .and_then(|src| src.get_mut(..src.len()))
        .ok_or(Error::InvalidOpCode)?
        .as_mut_ptr();
    unsafe { core::ptr::copy_nonoverlapping(src.as_ptr(), dest, src.len()) };
    Ok(())
}

#[allow(dead_code)]
pub fn memr<const N: usize>(dest: &mut [u8], src: &[u8], addr: usize) -> Result<(), Error> {
    let src = src
        .get(addr..)
        .and_then(|src| src.get(..dest.len()))
        .ok_or(Error::InvalidOpCode)?
        .as_ptr();
    unsafe { core::ptr::copy_nonoverlapping(src, dest.as_mut_ptr(), dest.len()) };
    Ok(())
}

#[inline(always)]
pub fn memr32(src: &[u8], addr: usize) -> Result<[u8; 4], Error> {
    read::<[u8; 4]>(src, addr)
}

#[allow(dead_code)]
#[inline(always)]
pub fn memr16(src: &[u8], addr: usize) -> Result<[u8; 2], Error> {
    read::<[u8; 2]>(src, addr)
}

#[allow(dead_code)]
#[inline(always)]
pub fn memr8(src: &[u8], addr: usize) -> Result<u8, Error> {
    read::<[u8; 1]>(src, addr).map(|n| unsafe { core::mem::transmute(n) })
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
        assert_eq!(read, b'h');
    }
}
