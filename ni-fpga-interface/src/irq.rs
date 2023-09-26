//! The IRQ module implements handling for interrupts to and from the FPGA.

use std::{fmt::Debug, os::raw::c_void, time::Duration};

use crate::{
    error::{FPGAError, NiFpgaStatus},
    session::{Session, SessionHandle},
    types::FpgaBool,
};

type IrqContextHandle = *const c_void;

extern "C" {
    fn NiFpga_ReserveIrqContext(
        session: SessionHandle,
        irq_context: *mut IrqContextHandle,
    ) -> NiFpgaStatus;

    fn NiFpga_UnreserveIrqContext(
        session: SessionHandle,
        irq_context: IrqContextHandle,
    ) -> NiFpgaStatus;

    fn NiFpga_WaitOnIrqs(
        session: SessionHandle,
        irq_context: IrqContextHandle,
        irqs: IrqSelection,
        timeout_ms: u32,
        irqs_asserted: *mut IrqSelection,
        timed_out: *mut FpgaBool,
    ) -> NiFpgaStatus;

    fn NiFpga_AcknowledgeIrqs(session: SessionHandle, irqs: IrqSelection) -> NiFpgaStatus;

}

/// IrqContext provides a single-threaded context in which you can wait
/// for specified IRQs.
///
/// This should be created from the session using [`Session::create_irq_context`].
///
/// The context is unreserved when it is dropped.
pub struct IrqContext<'session> {
    handle: IrqContextHandle,
    session: &'session SessionHandle,
}

/// Represents the result of a wait on IRQs.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IrqWaitResult {
    /// The wait timed out.
    TimedOut,
    /// The wait completed and the specified IRQs were asserted.
    IrqsAsserted(IrqSelection),
}

impl<'session> IrqContext<'session> {
    /// Wait on the specified IRQs for the specified timeout.
    ///
    /// See [`IrqSelection`] for details on setting specific IRQs.
    pub fn wait_on_irq(
        &mut self,
        irq: IrqSelection,
        timeout: Duration,
    ) -> Result<IrqWaitResult, FPGAError> {
        let mut irqs_asserted: IrqSelection = IrqSelection::NONE;
        let mut timed_out: FpgaBool = FpgaBool::FALSE;

        unsafe {
            let status = NiFpga_WaitOnIrqs(
                *self.session,
                self.handle,
                irq,
                timeout.as_millis() as u32,
                &mut irqs_asserted,
                &mut timed_out,
            );

            if status.is_error() {
                return Err(status.into());
            }
        }

        if timed_out == FpgaBool::TRUE {
            Ok(IrqWaitResult::TimedOut)
        } else {
            Ok(IrqWaitResult::IrqsAsserted(irqs_asserted))
        }
    }
}

impl Drop for IrqContext<'_> {
    fn drop(&mut self) {
        unsafe {
            NiFpga_UnreserveIrqContext(*self.session, self.handle);
        }
    }
}

impl Session {
    /// Creates an IRQ Context for the session.
    ///
    /// You can create multiple contexts but each context can only be used by one thread at a time.
    ///
    /// The context is then used to wait on specific IRQs. See [`IrqContext`].
    ///
    /// To minimize jitter when first waiting on IRQs, reserve as many contexts as the application requires.
    pub fn create_irq_context(&self) -> Result<IrqContext, FPGAError> {
        let mut handle: IrqContextHandle = std::ptr::null();
        unsafe {
            let status = NiFpga_ReserveIrqContext(self.handle, &mut handle);

            if status.is_error() {
                return Err(status.into());
            }
        }
        Ok(IrqContext {
            handle,
            session: &self.handle,
        })
    }

    /// Acknowledge the specified IRQs. See [`IrqSelection`] for details on setting specific IRQs.
    pub fn acknowledge_irqs(&self, irqs: IrqSelection) -> Result<(), FPGAError> {
        unsafe {
            let status = NiFpga_AcknowledgeIrqs(self.handle, irqs);

            if status.is_error() {
                return Err(status.into());
            }
        }
        Ok(())
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

impl Debug for IrqSelection {
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

pub const IRQ0: IrqSelection = IrqSelection::new(0);
pub const IRQ1: IrqSelection = IrqSelection::new(1);
pub const IRQ2: IrqSelection = IrqSelection::new(2);
pub const IRQ3: IrqSelection = IrqSelection::new(3);
pub const IRQ4: IrqSelection = IrqSelection::new(4);
pub const IRQ5: IrqSelection = IrqSelection::new(5);
pub const IRQ6: IrqSelection = IrqSelection::new(6);
pub const IRQ7: IrqSelection = IrqSelection::new(7);
pub const IRQ8: IrqSelection = IrqSelection::new(8);
pub const IRQ9: IrqSelection = IrqSelection::new(9);
pub const IRQ10: IrqSelection = IrqSelection::new(10);
pub const IRQ11: IrqSelection = IrqSelection::new(11);
pub const IRQ12: IrqSelection = IrqSelection::new(12);
pub const IRQ13: IrqSelection = IrqSelection::new(13);
pub const IRQ14: IrqSelection = IrqSelection::new(14);
pub const IRQ15: IrqSelection = IrqSelection::new(15);
pub const IRQ16: IrqSelection = IrqSelection::new(16);
pub const IRQ17: IrqSelection = IrqSelection::new(17);
pub const IRQ18: IrqSelection = IrqSelection::new(18);
pub const IRQ19: IrqSelection = IrqSelection::new(19);
pub const IRQ20: IrqSelection = IrqSelection::new(20);
pub const IRQ21: IrqSelection = IrqSelection::new(21);
pub const IRQ22: IrqSelection = IrqSelection::new(22);
pub const IRQ23: IrqSelection = IrqSelection::new(23);
pub const IRQ24: IrqSelection = IrqSelection::new(24);
pub const IRQ25: IrqSelection = IrqSelection::new(25);
pub const IRQ26: IrqSelection = IrqSelection::new(26);
pub const IRQ27: IrqSelection = IrqSelection::new(27);
pub const IRQ28: IrqSelection = IrqSelection::new(28);
pub const IRQ29: IrqSelection = IrqSelection::new(29);
pub const IRQ30: IrqSelection = IrqSelection::new(30);
pub const IRQ31: IrqSelection = IrqSelection::new(31);

#[cfg(test)]
mod tests {

    use super::IrqSelection;

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
