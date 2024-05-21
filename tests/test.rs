use std::mem::size_of;
use getrandom::getrandom;
use intsplit::*;


fn decompose<T: Sized>(data: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * std::mem::size_of::<T>())
    }
}

macro_rules! t16 {
    ($x:ty, $tt:ident) => {
        struct $tt {

        }

        impl $tt{
            pub fn run16() {
                let mut buf = vec![0u8; size_of::<$x>()];
                getrandom(buf.as_mut_slice()).expect("Fail");
                let dta = <$x>::from_ne_bytes(buf.clone().try_into().unwrap());
                let arr = dta.as_i8_array();
                let res = decompose(arr.as_slice());
                assert_eq!(buf.as_slice(), res);
            }
        }
    };
}

macro_rules! t32 {
    ($x:ty, $tt:ident) => {
        t16!($x, $tt);

        impl $tt {
            pub fn run32() {
                <$tt>::run16();
                let mut buf = vec![0u8; size_of::<$x>()];
                getrandom(buf.as_mut_slice()).expect("Fail");
                let dta = <$x>::from_ne_bytes(buf.clone().try_into().unwrap());
                let arr = dta.as_u16_array();
                let res = decompose(arr.as_slice());
                assert_eq!(buf.as_slice(), res);

                let mut buf = vec![0u8; size_of::<$x>()];
                getrandom(buf.as_mut_slice()).expect("Fail");
                let dta = <$x>::from_ne_bytes(buf.clone().try_into().unwrap());
                let arr = dta.as_i16_array();
                let res = decompose(arr.as_slice());
                assert_eq!(buf.as_slice(), res);

            }
        }
    };
}

macro_rules! t64 {
    ($x:ty, $tt:ident) => {
        t32!($x, $tt);

        impl $tt {
            pub fn run64() {
                <$tt>::run32();
                let mut buf = vec![0u8; size_of::<$x>()];
                getrandom(buf.as_mut_slice()).expect("Fail");
                let dta = <$x>::from_ne_bytes(buf.clone().try_into().unwrap());
                let arr = dta.as_u32_array();
                let res = decompose(arr.as_slice());
                assert_eq!(buf.as_slice(), res);

                let mut buf = vec![0u8; size_of::<$x>()];
                getrandom(buf.as_mut_slice()).expect("Fail");
                let dta = <$x>::from_ne_bytes(buf.clone().try_into().unwrap());
                let arr = dta.as_i32_array();
                let res = decompose(arr.as_slice());
                assert_eq!(buf.as_slice(), res);

                let mut buf = vec![0u8; size_of::<$x>()];
                getrandom(buf.as_mut_slice()).expect("Fail");
                let dta = <$x>::from_ne_bytes(buf.clone().try_into().unwrap());
                let arr = dta.as_f32_array();
                let res = decompose(arr.as_slice());
                assert_eq!(buf.as_slice(), res);

            }
        }
    };
}

macro_rules! t128 {
    ($x:ty, $tt:ident) => {
        t64!($x, $tt);

        impl $tt {
            pub fn run128() {
                <$tt>::run64();
                let mut buf = vec![0u8; size_of::<$x>()];
                getrandom(buf.as_mut_slice()).expect("Fail");
                let dta = <$x>::from_ne_bytes(buf.clone().try_into().unwrap());
                let arr = dta.as_u64_array();
                let res = decompose(arr.as_slice());
                assert_eq!(buf.as_slice(), res);

                let mut buf = vec![0u8; size_of::<$x>()];
                getrandom(buf.as_mut_slice()).expect("Fail");
                let dta = <$x>::from_ne_bytes(buf.clone().try_into().unwrap());
                let arr = dta.as_f64_array();
                let res = decompose(arr.as_slice());
                assert_eq!(buf.as_slice(), res);

            }
        }
    };
}


t16!(u16, U16T);
t16!(i16, I16T);
t32!(u32, U32T);
t32!(i32, I32T);
t32!(f32, F32T);
t64!(u64, U64T);
t64!(i64, I64T);
t64!(f64, F64T);
t128!(i128, I128T);
t128!(u128, U128T);

#[test]
pub fn test_u16() {
    for _ in 0..100 {
        U16T::run16();
    }
}

#[test]
pub fn test_i16() {
    for _ in 0..100 {
        I16T::run16();
    }
}


#[test]
pub fn test_u32() {
    for _ in 0..100 {
        U32T::run32();
    }
}

#[test]
pub fn test_i32() {
    for _ in 0..100 {
        I32T::run32();
    }
}

#[test]
pub fn test_f32() {
    for _ in 0..100 {
        F32T::run32();
    }
}


#[test]
pub fn test_u64() {
    for _ in 0..100 {
        U64T::run64();
    }
}

#[test]
pub fn test_i64() {
    for _ in 0..100 {
        I64T::run64();
    }
}

#[test]
pub fn test_f64() {
    for _ in 0..100 {
        F64T::run64();
    }
}
#[test]
pub fn test_u128() {
    for _ in 0..100 {
        U128T::run128();
    }
}

#[test]
pub fn test_i128() {
    for _ in 0..100 {
        I128T::run128();
    }
}


#[test]
fn example() {
    let number : u32 = 0x0A0B0C0Du32;
    let u16_components: [u16; 2] = number.as_u16_array();

    #[cfg(target_endian = "little")]
    assert_eq!([0x0C0Du16, 0x0A0Bu16], u16_components);

    #[cfg(target_endian = "big")]
    assert_eq!([0x0A0Bu16, 0x0C0Du16], u16_components);
}

