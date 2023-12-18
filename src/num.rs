#[allow(clippy::missing_safety_doc)]
pub unsafe trait Unsigned: Bitcast<Self::Signed> + As<Self::Signed> + Sized {
    type Signed: Sized + Bitcast<Self> + As<Self>;
}

pub trait As<Shr = Self>: Sized {
    fn r#as(self) -> Shr;
}

pub trait Bitcast<Shr = Self>: Sized {
    fn bitcast(self) -> Shr;
}

pub trait Wrapping: Sized {
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_shl(self, rhs: u32) -> Self;
    fn wrapping_shr(self, rhs: u32) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn wrapping_rem(self, rhs: Self) -> Self;
}

pub trait Widening: Sized {
    fn widening_mul(self, rhs: Self) -> (Self, Self);
}

pub trait UnsignedWrapping: Wrapping + Unsigned {
    fn wrapping_add_signed(self, rhs: Self::Signed) -> Self;
}

macro_rules! impl_wrapping {
    ($t:ty) => {
        impl Wrapping for $t {
            #[inline(always)]
            fn wrapping_add(self, rhs: Self) -> Self {
                <$t>::wrapping_add(self, rhs)
            }

            #[inline(always)]
            fn wrapping_sub(self, rhs: Self) -> Self {
                <$t>::wrapping_sub(self, rhs)
            }

            #[inline(always)]
            fn wrapping_shl(self, rhs: u32) -> Self {
                <$t>::wrapping_shl(self, rhs)
            }

            #[inline(always)]
            fn wrapping_shr(self, rhs: u32) -> Self {
                <$t>::wrapping_shr(self, rhs)
            }

            #[inline(always)]
            fn wrapping_mul(self, rhs: Self) -> Self {
                <$t>::wrapping_mul(self, rhs)
            }

            #[inline(always)]
            fn wrapping_div(self, rhs: Self) -> Self {
                <$t>::wrapping_div(self, rhs)
            }

            #[inline(always)]
            fn wrapping_rem(self, rhs: Self) -> Self {
                <$t>::wrapping_rem(self, rhs)
            }
        }
    };
}

macro_rules! impl_widening {
    ($t:ty, $wide_t:ty, $bits:literal) => {
        impl Widening for $t {
            #[inline(always)]
            fn widening_mul(self, rhs: Self) -> (Self, Self) {
                let wide = (self as $wide_t) * (rhs as $wide_t);
                (wide as $t, (wide >> $bits) as $t)
            }
        }
    };
}

macro_rules! impl_wrapping_unsigned {
    ($t:ty, $st:ty) => {
        impl UnsignedWrapping for $t {
            #[inline(always)]
            fn wrapping_add_signed(self, rhs: $st) -> Self {
                <$t>::wrapping_add_signed(self, rhs)
            }
        }
    };
}

impl_wrapping!(u32);
impl_wrapping!(u64);
impl_wrapping!(i32);
impl_wrapping!(i64);
impl_widening!(u32, u64, 32);
impl_widening!(u64, u128, 64);
impl_widening!(i32, i64, 32);
impl_widening!(i64, i128, 64);
impl_wrapping_unsigned!(u32, i32);
impl_wrapping_unsigned!(u64, i64);

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
