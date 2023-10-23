//! The IRQ module implements handling for interrupts to and from the FPGA.

use std::{fmt::Debug, time::Duration};

use crate::{error::FPGAError, nifpga_sys::*, session::Session, types::FpgaBool};

// Re-excport the IRQ selection types from here for a better dev experience.
pub use crate::types::IrqSelection;

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
