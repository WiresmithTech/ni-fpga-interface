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

/// Represents a selection of 0 or more IRQs.
///
/// Internally the FPGA API expects a bitwise field to select IRQs.
/// This type wraps that representation with functions to help make
/// different selections and conversions.
///
/// For single IRQs you can use [`IrqSelection::new`] and to query if they are set use [`IrqSelection::is_irq_set`].
///
/// ```
/// use ni_fpga_interface::irq::IrqSelection;
/// let selection = IrqSelection::new(2);
/// assert_eq!(selection.is_irq_set(2), true);
/// ```
///
/// There is also an iterator method if you want to cycle through the set IRQs.
/// ```
/// use ni_fpga_interface::irq::IrqSelection;
/// let mut selection = IrqSelection::new(0);
/// selection.add_irq(2);
/// for irq in selection.iter() {
///    println!("IRQ: {}", irq);
/// }
/// ```
///
/// To mimic the C API the IRQs are also available as constants.
/// ```
/// use ni_fpga_interface::irq::{IRQ1};
/// assert_eq!(IRQ1.is_irq_set(1), true);
/// ```
///
///
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IrqSelection(u32);

impl IrqSelection {
    /// Represents a selection with no IRQs.
    pub const NONE: IrqSelection = IrqSelection(0);

    /// Creates a new IRQ selection with the specified IRQ included.
    pub const fn new(irq: u8) -> IrqSelection {
        IrqSelection(1 << irq)
    }

    /// Add an IRQ number by selection.
    pub fn add_irq(&mut self, irq: u8) {
        self.0 |= IrqSelection::new(irq).0;
    }

    /// Check if an IRQ number is set.
    pub fn is_irq_set(&self, irq: u8) -> bool {
        self.0 & IrqSelection::new(irq).0 != 0
    }

    /// Iterate over the IRQs that are set.
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        (0..32).filter(|irq| self.is_irq_set(*irq))
    }
}

impl Default for IrqSelection {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Debug for IrqSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IrqSelection[")?;
        let mut first = true;
        for irq in self.iter() {
            if first {
                write!(f, "{}", irq)?;
            } else {
                write!(f, ", {}", irq)?;
            }
            first = false;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl From<u32> for IrqSelection {
    /// Intialize an IRQ selection from a raw value.
    fn from(value: u32) -> Self {
        IrqSelection(value)
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

    #[test]
    fn test_irq_selection_new() {
        let irq = IrqSelection::new(20);
        assert_eq!(irq.0, 1 << 20);
    }

    #[test]
    fn test_add_irq_to_selection() {
        let mut selection = IrqSelection::new(0);
        selection.add_irq(2);
        assert_eq!(selection.0, 0b101);
    }

    #[test]
    fn test_check_if_irq_is_set() {
        let mut selection = IrqSelection::new(0);
        selection.add_irq(2);
        assert_eq!(selection.is_irq_set(2), true);
        assert_eq!(selection.is_irq_set(1), false);
    }

    #[test]
    fn iterate_over_set_values() {
        let mut selection = IrqSelection::new(0);
        selection.add_irq(2);
        let set_values = selection.iter().collect::<Vec<_>>();
        assert_eq!(set_values, vec![0, 2])
    }

    #[test]
    fn test_default() {
        let selection = IrqSelection::default();
        assert_eq!(selection.0, 0);
    }

    #[test]
    fn test_irq_selection_display() {
        let mut selection = IrqSelection::new(0);
        selection.add_irq(2);
        assert_eq!(format!("{:?}", selection), "IrqSelection[0, 2]");
    }
}
