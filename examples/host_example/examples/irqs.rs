//! This example demonstrates how to use IRQs with the host API.
//!
//! The FPGA will loop around with the following steps:
//!
//! 1. Assert IRQ0
//! 2. Wait on IRQ0 acknowledgement.
//! 3. Increment the `IRQs` register.

use ni_fpga_interface::irq::{IrqWaitResult, IRQ0};
use std::time::Duration;

mod fpga_defs {
    include!(concat!(env!("OUT_DIR"), "/NiFpga_Main.rs"));
}

fn main() {
    let session = host_example::connect_fpga();

    //Test IRQs.
    let mut irq_context = session.create_irq_context().unwrap();
    let irq_count_reg = fpga_defs::registers::IRQs;

    let result = irq_context
        .wait_on_irq(IRQ0, Duration::from_millis(1000))
        .unwrap();
    match result {
        IrqWaitResult::TimedOut => println!("IRQ Timed Out"),
        IrqWaitResult::IrqsAsserted(irqs) => {
            println!("IRQs Asserted: {:?}", irqs);
            assert!(irq_count_reg.read(&session).unwrap() == 0);
            session.acknowledge_irqs(IRQ0).unwrap();
            assert!(irq_count_reg.read(&session).unwrap() == 1);
        }
    }
}
