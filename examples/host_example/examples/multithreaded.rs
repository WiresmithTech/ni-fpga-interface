//! This is designed to be a reference example for how multithreading should work
//! in the host API.
//!
//! In this case we simply wrap the session in an Arc and pass it to the threads.
//!
//! We cannot clone the session as it will attempt to drop and close the session for each clone.
//!
//! Review: Should we just build this into the session?

use ni_fpga_interface::{
    irq::{IrqWaitResult, IRQ0},
    session::Session,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

mod fpga_defs {
    include!(concat!(env!("OUT_DIR"), "/NiFpga_Main.rs"));
}

/// This thread will use the IRQs and a single register.
fn irq_thread(session: Arc<Session>, stop: Arc<AtomicBool>) {
    println!("IRQ thread started");

    let mut irq_context = session.create_irq_context().unwrap();
    let irq_count_reg = fpga_defs::registers::IRQs;

    // Here I will grab the session as a reference for use below
    // to save repeating as ref as the register interface can't
    // automatically dereference the arc.
    let session = session.as_ref();

    let mut count = 0;

    while !stop.load(Ordering::Relaxed) {
        let result = irq_context
            .wait_on_irq(IRQ0, Duration::from_millis(1000))
            .unwrap();
        match result {
            IrqWaitResult::TimedOut => println!("IRQ Timed Out"),
            IrqWaitResult::IrqsAsserted(_irqs) => {
                session.acknowledge_irqs(IRQ0).unwrap();
                count += 1;
                let value = irq_count_reg.read(session).unwrap();
                assert_eq!(value, count);
            }
        }
        sleep(Duration::from_millis(100));
    }

    println!("IRQ thread done. Iterations: {count}");
}

/// This thread will exercise the registers API.
fn regs_thread(session: Arc<Session>, stop: Arc<AtomicBool>) {
    println!("Regs thread started");

    let output_reg = fpga_defs::registers::U8Control;
    let input_1_reg = fpga_defs::registers::U8Result;

    let session = session.as_ref();

    let mut count = 0;

    while !stop.load(Ordering::Relaxed) {
        count += 1;
        output_reg.write(session, count).unwrap();
        let value = input_1_reg.read(session).unwrap();
        assert!(value == count);
        sleep(Duration::from_millis(100));
    }
    println!("Regs thread done. Iterations: {count}");
}

fn main() {
    let session = host_example::connect_fpga();
    let session_shared = Arc::new(session);

    let stop = Arc::new(AtomicBool::new(false));

    let irq_stop = stop.clone();
    let irq_session = session_shared.clone();
    let irq_thread = std::thread::spawn(move || {
        irq_thread(irq_session, irq_stop);
    });

    let regs_stop = stop.clone();
    let reg_session = session_shared.clone();
    let regs_thread = std::thread::spawn(move || {
        regs_thread(reg_session, regs_stop);
    });

    sleep(Duration::from_millis(10000));
    stop.store(true, Ordering::Relaxed);
    irq_thread.join().unwrap();
    regs_thread.join().unwrap();
}
