/// Wrapper for any types shared across the interface.
///
use std::time::Duration;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FpgaBool(u8);

impl FpgaBool {
    pub const TRUE: FpgaBool = FpgaBool(1);
    pub const FALSE: FpgaBool = FpgaBool(0);
}

/// Wrapper for the FpgaTimeout fields to handle
/// the conversion from Duration and handling
/// infinite timeouts.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FpgaTimeoutMs(u32);

impl FpgaTimeoutMs {
    pub const INFINITE: FpgaTimeoutMs = FpgaTimeoutMs(0xFFFFFFFF);
}

impl From<Duration> for FpgaTimeoutMs {
    fn from(duration: Duration) -> Self {
        FpgaTimeoutMs(duration.as_millis() as u32)
    }
}

impl From<Option<Duration>> for FpgaTimeoutMs {
    fn from(duration: Option<Duration>) -> Self {
        match duration {
            Some(duration) => duration.into(),
            None => FpgaTimeoutMs::INFINITE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inifinite_timeout_representation() {
        assert_eq!(FpgaTimeoutMs::INFINITE, FpgaTimeoutMs(0xFFFFFFFF));
    }

    #[test]
    fn test_timeout_conversion_optional() {
        assert_eq!(FpgaTimeoutMs::from(None), FpgaTimeoutMs(0xFFFFFFFF));
        assert_eq!(
            FpgaTimeoutMs::from(Some(Duration::from_millis(100))),
            FpgaTimeoutMs(100)
        );
    }
}
