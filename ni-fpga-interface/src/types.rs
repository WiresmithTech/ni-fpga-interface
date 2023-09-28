/// Wrapper for any types shared across the interface.
///
use std::time::Duration;

/// The representation of a boolean in the FPGA interface.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FpgaBool(u8);

impl FpgaBool {
    pub const TRUE: FpgaBool = FpgaBool(1);
    pub const FALSE: FpgaBool = FpgaBool(0);
}

impl From<bool> for FpgaBool {
    fn from(value: bool) -> Self {
        if value {
            FpgaBool::TRUE
        } else {
            FpgaBool::FALSE
        }
    }
}

impl From<FpgaBool> for bool {
    fn from(value: FpgaBool) -> Self {
        value.0 != 0
    }
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

    #[test]
    fn test_fpga_bool_into_bool() {
        let bool_true: bool = FpgaBool::TRUE.into();
        let bool_false: bool = FpgaBool::FALSE.into();
        assert_eq!(bool_true, true);
        assert_eq!(bool_false, false);
    }

    #[test]
    fn test_bool_into_fpga_bool() {
        let fpga_bool_true: FpgaBool = true.into();
        let fpga_bool_false: FpgaBool = false.into();
        assert_eq!(fpga_bool_true, FpgaBool::TRUE);
        assert_eq!(fpga_bool_false, FpgaBool::FALSE);
    }
}
