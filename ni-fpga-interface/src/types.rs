/// Wrapper for any types shared across the interface.

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FpgaBool(u8);

impl FpgaBool {
    pub const TRUE: FpgaBool = FpgaBool(1);
    pub const FALSE: FpgaBool = FpgaBool(0);
}
