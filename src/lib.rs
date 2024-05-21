#![no_std]

pub trait Split16 : private::Sealed + Sized {
    fn as_u8_array(self) -> [u8; 2];

    #[inline]
    fn as_i8_array(self) -> [i8; 2] {
        let n = self.as_u8_array();
        return [
            n[0] as i8,
            n[1] as i8
        ]
    }
}

impl Split16 for u16 {

    #[inline]
    fn as_u8_array(self) -> [u8; 2] {
        self.to_ne_bytes()
    }
}

impl Split16 for i16 {

    #[inline]
    fn as_u8_array(self) -> [u8; 2] {
        self.to_ne_bytes()
    }
}

pub trait Split32 : private::Sealed + Sized {
    fn as_u8_array(self) -> [u8; 4];

    #[inline]
    fn as_i8_array(self) -> [i8; 4] {
        let n = self.as_u8_array();
        return [
            n[0] as i8,
            n[1] as i8,
            n[2] as i8,
            n[3] as i8
        ]
    }

    #[inline]
    fn as_u16_array(self) -> [u16; 2] {
        let n = self.as_u8_array();
        return [
            u16::from_ne_bytes([n[0], n[1]]),
            u16::from_ne_bytes([n[2], n[3]]),
        ]
    }

    #[inline]
    fn as_i16_array(self) -> [i16; 2] {
        let n = self.as_u8_array();
        return [
            i16::from_ne_bytes([n[0], n[1]]),
            i16::from_ne_bytes([n[2], n[3]]),
        ]
    }
}

impl Split32 for u32 {

    #[inline]
    fn as_u8_array(self) -> [u8; 4] {
        self.to_ne_bytes()
    }
}

impl Split32 for i32 {

    #[inline]
    fn as_u8_array(self) -> [u8; 4] {
        self.to_ne_bytes()
    }
}

impl Split32 for f32 {

    #[inline]
    fn as_u8_array(self) -> [u8; 4] {
        self.to_ne_bytes()
    }
}

pub trait Split64 : private::Sealed + Sized {
    fn as_u8_array(self) -> [u8; 8];

    #[inline]
    fn as_i8_array(self) -> [i8; 8] {
        let n = self.as_u8_array();
        return [
            n[0] as i8,
            n[1] as i8,
            n[2] as i8,
            n[3] as i8,
            n[4] as i8,
            n[5] as i8,
            n[6] as i8,
            n[7] as i8,
        ]
    }

    #[inline]
    fn as_u16_array(self) -> [u16; 4] {
        let n = self.as_u8_array();
        return [
            u16::from_ne_bytes([n[0], n[1]]),
            u16::from_ne_bytes([n[2], n[3]]),
            u16::from_ne_bytes([n[4], n[5]]),
            u16::from_ne_bytes([n[6], n[7]]),
        ]
    }

    #[inline]
    fn as_i16_array(self) -> [i16; 4] {
        let n = self.as_u8_array();
        return [
            i16::from_ne_bytes([n[0], n[1]]),
            i16::from_ne_bytes([n[2], n[3]]),
            i16::from_ne_bytes([n[4], n[5]]),
            i16::from_ne_bytes([n[6], n[7]]),
        ]
    }

    #[inline]
    fn as_u32_array(self) -> [u32; 2] {
        let n = self.as_u8_array();
        return [
            u32::from_ne_bytes([n[0], n[1], n[2], n[3]]),
            u32::from_ne_bytes([n[4], n[5], n[6], n[7]]),
        ]
    }

    #[inline]
    fn as_i32_array(self) -> [i32; 2] {
        let n = self.as_u8_array();
        return [
            i32::from_ne_bytes([n[0], n[1], n[2], n[3]]),
            i32::from_ne_bytes([n[4], n[5], n[6], n[7]]),
        ]
    }

    #[inline]
    fn as_f32_array(self) -> [f32; 2] {
        let n = self.as_u8_array();
        return [
            f32::from_ne_bytes([n[0], n[1], n[2], n[3]]),
            f32::from_ne_bytes([n[4], n[5], n[6], n[7]]),
        ]
    }
}

impl Split64 for u64 {

    #[inline]
    fn as_u8_array(self) -> [u8; 8] {
        self.to_ne_bytes()
    }
}

impl Split64 for i64 {

    #[inline]
    fn as_u8_array(self) -> [u8; 8] {
        self.to_ne_bytes()
    }
}

impl Split64 for f64 {

    #[inline]
    fn as_u8_array(self) -> [u8; 8] {
        self.to_ne_bytes()
    }
}

pub trait Split128 : private::Sealed + Sized {
    fn as_u8_array(self) -> [u8; 16];


    #[inline]
    fn as_i8_array(self) -> [i8; 16] {
        let n = self.as_u8_array();
        return [
            n[0] as i8,
            n[1] as i8,
            n[2] as i8,
            n[3] as i8,
            n[4] as i8,
            n[5] as i8,
            n[6] as i8,
            n[7] as i8,
            n[8] as i8,
            n[9] as i8,
            n[10] as i8,
            n[11] as i8,
            n[12] as i8,
            n[13] as i8,
            n[14] as i8,
            n[15] as i8,
        ]
    }

    #[inline]
    fn as_u16_array(self) -> [u16; 8] {
        let n = self.as_u8_array();
        return [
            u16::from_ne_bytes([n[0], n[1]]),
            u16::from_ne_bytes([n[2], n[3]]),
            u16::from_ne_bytes([n[4], n[5]]),
            u16::from_ne_bytes([n[6], n[7]]),
            u16::from_ne_bytes([n[8], n[9]]),
            u16::from_ne_bytes([n[10], n[11]]),
            u16::from_ne_bytes([n[12], n[13]]),
            u16::from_ne_bytes([n[14], n[15]]),
        ]
    }

    #[inline]
    fn as_i16_array(self) -> [i16; 8] {
        let n = self.as_u8_array();
        return [
            i16::from_ne_bytes([n[0], n[1]]),
            i16::from_ne_bytes([n[2], n[3]]),
            i16::from_ne_bytes([n[4], n[5]]),
            i16::from_ne_bytes([n[6], n[7]]),
            i16::from_ne_bytes([n[8], n[9]]),
            i16::from_ne_bytes([n[10], n[11]]),
            i16::from_ne_bytes([n[12], n[13]]),
            i16::from_ne_bytes([n[14], n[15]]),
        ]
    }

    #[inline]
    fn as_u32_array(self) -> [u32; 4] {
        let n = self.as_u8_array();
        return [
            u32::from_ne_bytes([n[0], n[1], n[2], n[3]]),
            u32::from_ne_bytes([n[4], n[5], n[6], n[7]]),
            u32::from_ne_bytes([n[8], n[9], n[10], n[11]]),
            u32::from_ne_bytes([n[12], n[13], n[14], n[15]]),
        ]
    }

    #[inline]
    fn as_i32_array(self) -> [i32; 4] {
        let n = self.as_u8_array();
        return [
            i32::from_ne_bytes([n[0], n[1], n[2], n[3]]),
            i32::from_ne_bytes([n[4], n[5], n[6], n[7]]),
            i32::from_ne_bytes([n[8], n[9], n[10], n[11]]),
            i32::from_ne_bytes([n[12], n[13], n[14], n[15]]),
        ]
    }

    #[inline]
    fn as_f32_array(self) -> [f32; 4] {
        let n = self.as_u8_array();
        return [
            f32::from_ne_bytes([n[0], n[1], n[2], n[3]]),
            f32::from_ne_bytes([n[4], n[5], n[6], n[7]]),
            f32::from_ne_bytes([n[8], n[9], n[10], n[11]]),
            f32::from_ne_bytes([n[12], n[13], n[14], n[15]]),
        ]
    }

    #[inline]
    fn as_u64_array(self) -> [u64; 2] {
        let n = self.as_u8_array();
        return [
            u64::from_ne_bytes([n[0], n[1], n[2], n[3],n[4], n[5], n[6], n[7]]),
            u64::from_ne_bytes([n[8], n[9], n[10], n[11],n[12], n[13], n[14], n[15]]),
        ]
    }

    #[inline]
    fn as_i64_array(self) -> [i64; 2] {
        let n = self.as_u8_array();
        return [
            i64::from_ne_bytes([n[0], n[1], n[2], n[3],n[4], n[5], n[6], n[7]]),
            i64::from_ne_bytes([n[8], n[9], n[10], n[11],n[12], n[13], n[14], n[15]]),
        ]
    }

    #[inline]
    fn as_f64_array(self) -> [f64; 2] {
        let n = self.as_u8_array();
        return [
            f64::from_ne_bytes([n[0], n[1], n[2], n[3],n[4], n[5], n[6], n[7]]),
            f64::from_ne_bytes([n[8], n[9], n[10], n[11],n[12], n[13], n[14], n[15]]),
        ]
    }
}

impl Split128 for u128 {

    #[inline]
    fn as_u8_array(self) -> [u8; 16] {
        self.to_ne_bytes()
    }
}

impl Split128 for i128 {

    #[inline]
    fn as_u8_array(self) -> [u8; 16] {
        self.to_ne_bytes()
    }
}

pub(crate) mod private {
    pub trait Sealed {

    }

    impl Sealed for u16 {

    }
    impl Sealed for i16 {

    }
    impl Sealed for u32 {

    }
    impl Sealed for i32 {

    }
    impl Sealed for u64 {

    }
    impl Sealed for i64 {

    }
    impl Sealed for u128 {

    }
    impl Sealed for i128 {

    }
    impl Sealed for f32 {

    }
    impl Sealed for f64 {

    }
}