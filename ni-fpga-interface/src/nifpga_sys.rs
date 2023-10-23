use crate::error::NiFpgaStatus;
use crate::types::{FpgaBool, FpgaTimeoutMs, IrqSelection};
use libc::{c_char, c_void, size_t};
use paste::paste;

pub type SessionHandle = u32;

pub type FifoAddress = u32;
pub type PeerToPeerEndpoint = u32;

pub type IrqContextHandle = *const c_void;

/// First entry is the rust type, second is the text used for that type in the FPGA interface.
macro_rules! impl_type_session_interface {
    ($rust_type:ty, $fpga_type:literal) => {

            paste! { pub fn [<NiFpga_Read $fpga_type >](session: SessionHandle, offset: u32, value: *mut $rust_type) -> NiFpgaStatus; }
            paste! { pub fn [<NiFpga_Write $fpga_type >](session: SessionHandle, offset: u32, value: $rust_type) -> NiFpgaStatus; }
            paste! { pub fn [<NiFpga_ReadArray $fpga_type >](session: SessionHandle, offset: u32, value: *mut $rust_type, size: size_t) -> NiFpgaStatus; }
            paste! { pub fn [<NiFpga_WriteArray $fpga_type >](session: SessionHandle, offset: u32, value: *const $rust_type, size: size_t) -> NiFpgaStatus; }
            paste! { pub fn [<NiFpga_ReadFifo $fpga_type >](session: SessionHandle, fifo: u32, data: *mut $rust_type, number_of_elements: size_t, timeout_ms: FpgaTimeoutMs, elements_remaining: *mut size_t) -> NiFpgaStatus; }
            paste! { pub fn [<NiFpga_WriteFifo $fpga_type >](session: SessionHandle, fifo: u32, data: *const $rust_type, number_of_elements: size_t, timeout_ms: FpgaTimeoutMs, elements_remaining: *mut size_t) -> NiFpgaStatus;}
            paste! { pub fn [<NiFpga_AcquireFifoReadElements $fpga_type >](session: SessionHandle, fifo: u32, elements: *mut *const $rust_type, elements_requested: size_t, timeout_ms: FpgaTimeoutMs, elements_acquired: *mut size_t, elements_remaining: *mut size_t) -> NiFpgaStatus; }
            paste! { pub fn [<NiFpga_AcquireFifoWriteElements $fpga_type >](session: SessionHandle, fifo: u32, elements: *mut *mut $rust_type, elements_requested: size_t, timeout_ms: FpgaTimeoutMs, elements_acquired: *mut size_t, elements_remaining: *mut size_t) -> NiFpgaStatus; }
    }
}

#[cfg_attr(not(test), link(name = "ni_fpga"))]
extern "C" {
    pub fn NiFpga_Initialize() -> NiFpgaStatus;
    pub fn NiFpga_Finalize() -> NiFpgaStatus;
    pub fn NiFpga_Open(
        bitfile: *const c_char,
        signature: *const c_char,
        resource: *const c_char,
        attribute: u32,
        session: *mut SessionHandle,
    ) -> NiFpgaStatus;
    pub fn NiFpga_Reset(session: SessionHandle) -> NiFpgaStatus;
    pub fn NiFpga_Abort(session: SessionHandle) -> NiFpgaStatus;
    pub fn NiFpga_Download(session: SessionHandle) -> NiFpgaStatus;
    pub fn NiFpga_Run(session: SessionHandle, attributes: u32) -> NiFpgaStatus;
    pub fn NiFpga_Close(session: SessionHandle, attribute: u32) -> NiFpgaStatus;

    pub fn NiFpga_ReserveIrqContext(
        session: SessionHandle,
        irq_context: *mut IrqContextHandle,
    ) -> NiFpgaStatus;

    pub fn NiFpga_UnreserveIrqContext(
        session: SessionHandle,
        irq_context: IrqContextHandle,
    ) -> NiFpgaStatus;

    pub fn NiFpga_WaitOnIrqs(
        session: SessionHandle,
        irq_context: IrqContextHandle,
        irqs: IrqSelection,
        timeout_ms: u32,
        irqs_asserted: *mut IrqSelection,
        timed_out: *mut FpgaBool,
    ) -> NiFpgaStatus;

    pub fn NiFpga_AcknowledgeIrqs(session: SessionHandle, irqs: IrqSelection) -> NiFpgaStatus;

    pub fn NiFpga_ConfigureFifo2(
        session: SessionHandle,
        fifo: FifoAddress,
        requested_depth: size_t,
        actual_depth: *mut size_t,
    ) -> NiFpgaStatus;

    pub fn NiFpga_StartFifo(session: SessionHandle, fifo: FifoAddress) -> NiFpgaStatus;

    pub fn NiFpga_StopFifo(session: SessionHandle, fifo: FifoAddress) -> NiFpgaStatus;

    pub fn NiFpga_ReleaseFifoElements(
        session: SessionHandle,
        fifo: FifoAddress,
        number_of_elements: size_t,
    ) -> NiFpgaStatus;

    pub fn NiFpga_GetPeerToPeerFifoEndpoint(
        session: SessionHandle,
        fifo: FifoAddress,
        endpoint: *mut PeerToPeerEndpoint,
    ) -> NiFpgaStatus;

    impl_type_session_interface!(u8, "U8");
    impl_type_session_interface!(u16, "U16");
    impl_type_session_interface!(u32, "U32");
    impl_type_session_interface!(u64, "U64");
    impl_type_session_interface!(i8, "I8");
    impl_type_session_interface!(i16, "I16");
    impl_type_session_interface!(i32, "I32");
    impl_type_session_interface!(i64, "I64");
    impl_type_session_interface!(f32, "Sgl");
    impl_type_session_interface!(f64, "Dbl");
    impl_type_session_interface!(FpgaBool, "Bool");

}
