/// Validate and deserialize unsigned integers.
pub trait ValidateUnsignedInteger: Sized {
    type Error;

    fn parse_u128(i: u128) -> Result<Self, Self::Error>;

    fn validate_u128(i: u128) -> Result<(), Self::Error>;

    #[cfg(not(any(
        target_pointer_width = "32",
        target_pointer_width = "16"
    )))]
    #[inline]
    fn parse_usize(i: usize) -> Result<Self, Self::Error> {
        Self::parse_u64(i as u64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn parse_usize(i: usize) -> Result<Self, Self::Error> {
        Self::parse_u32(i as u32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn parse_usize(i: usize) -> Result<Self, Self::Error> {
        Self::parse_u16(i as u16)
    }

    #[inline]
    fn parse_u64(i: u64) -> Result<Self, Self::Error> {
        Self::parse_u128(i as u128)
    }

    #[inline]
    fn parse_u32(i: u32) -> Result<Self, Self::Error> {
        Self::parse_u64(i as u64)
    }

    #[inline]
    fn parse_u16(i: u16) -> Result<Self, Self::Error> {
        Self::parse_u32(i as u32)
    }

    #[inline]
    fn parse_u8(i: u8) -> Result<Self, Self::Error> {
        Self::parse_u16(i as u16)
    }

    #[cfg(not(any(
        target_pointer_width = "32",
        target_pointer_width = "16"
    )))]
    #[inline]
    fn validate_usize(i: usize) -> Result<(), Self::Error> {
        Self::validate_u64(i as u64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn validate_usize(i: usize) -> Result<(), Self::Error> {
        Self::validate_u32(i as u32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn validate_usize(i: usize) -> Result<(), Self::Error> {
        Self::validate_u16(i as u16)
    }

    #[inline]
    fn validate_u64(i: u64) -> Result<(), Self::Error> {
        Self::validate_u128(i as u128)
    }

    #[inline]
    fn validate_u32(i: u32) -> Result<(), Self::Error> {
        Self::validate_u64(i as u64)
    }

    #[inline]
    fn validate_u16(i: u16) -> Result<(), Self::Error> {
        Self::validate_u32(i as u32)
    }

    #[inline]
    fn validate_u8(i: u8) -> Result<(), Self::Error> {
        Self::validate_u16(i as u16)
    }
}
