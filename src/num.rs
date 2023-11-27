#[allow(clippy::missing_safety_doc)]
pub unsafe trait Unsigned: Bitcast<Self::Signed> + As<Self::Signed> + Sized {
    type Signed: Bitcast<Self> + As<Self>;
}

pub trait As<Shr = Self>: Sized {
    fn r#as(self) -> Shr;
}

pub trait Bitcast<Shr = Self>: Sized {
    fn bitcast(self) -> Shr;
}

macro_rules! impl_as {
    ($t:ty => $($tt:ty),* $(,)?) => {
        impl As for $t {
            fn r#as(self) -> Self {
                self
            }
        }

        $(
            impl As<$tt> for $t {
                fn r#as(self) -> $tt {
                    self as $tt
                }
            }
        )*
    };
    ($($t:ty => ($($tt:ty),* $(,)?);)*) => {
        $(impl_as!($t => $($tt),*);)*
    };
}

impl_as! {
    i8 => (u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
    u8 => (i8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
    i16 => (i8, u8, u16, i32, u32, i64, u64, i128, u128, isize, usize);
    u16 => (i8, u8, i16, i32, u32, i64, u64, i128, u128, isize, usize);
    i32 => (i8, u8, i16, u16, u32, i64, u64, i128, u128, isize, usize);
    u32 => (i8, u8, i16, u16, i32, i64, u64, i128, u128, isize, usize);
    i64 => (i8, u8, i16, u16, i32, u32, u64, i128, u128, isize, usize);
    u64 => (i8, u8, i16, u16, i32, u32, i64, i128, u128, isize, usize);
    i128 => (i8, u8, i16, u16, i32, u32, i64, u64, u128, isize, usize);
    u128 => (i8, u8, i16, u16, i32, u32, i64, u64, i128, isize, usize);
    isize => (i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, usize);
    usize => (i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize);
    bool => (i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
}

macro_rules! impl_uint {
    (@bitcast $ut:ty => $st:ty) => {
        impl Bitcast for $ut {
            #[inline(always)]
            fn bitcast(self) -> Self {
                self
            }
        }

        impl Bitcast<$st> for $ut {
            #[inline(always)]
            fn bitcast(self) -> $st {
                unsafe { core::mem::transmute(self) }
            }
        }
    };
    ($ut:ty => $st:ty) => {
        unsafe impl Unsigned for $ut {
            type Signed = $st;
        }
        impl_uint!(@bitcast $ut => $st);
        impl_uint!(@bitcast $st => $ut);
    };
    ($($ut:ty => $st:ty;)*) => {
        $(impl_uint!($ut => $st);)*
    };
}

impl_uint! {
    u8 => i8;
    u16 => i16;
    u32 => i32;
    u64 => i64;
}
