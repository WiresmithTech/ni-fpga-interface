use ni_fpga_interface::{
    irq::{self, IRQ0},
    session::Session,
};
use std::{path::Path, time::Duration};

mod fpga_defs {
    include!(concat!(env!("OUT_DIR"), "/NiFpga_Main.rs"));
}

fn main() {
    let bitfile = Path::new("../fpga_c_interface/NiFpga_Main.lvbitx");
    let session = Session::new(
        bitfile.to_str().unwrap(),
        fpga_defs::SIGNATURE,
        "rio://192.168.10.17/RIO0",
    )
    .unwrap();
    //session.run().unwrap();

    let output_reg = fpga_defs::registers::U8Control;
    let input_1_reg = fpga_defs::registers::U8Result;
    let value = output_reg.read(&session).unwrap();
    println!("Value: {value}");
    input_1_reg.write(&session, 0x55).unwrap();
    let value = output_reg.read(&session).unwrap();
    println!("Value: {value}");

    let output_array = fpga_defs::registers::U8ControlArray;
    let input_array = fpga_defs::registers::U8ResultArray;
    output_array.write(&session, &[1u8, 2, 3, 4]).unwrap();
    let array: [u8; 4] = input_array.read(&session).unwrap();
    println!("Array: {:?}", array);

    //Test IRQs.
    let mut irq_context = session.create_irq_context().unwrap();
    let irq_count_reg = fpga_defs::registers::IRQs;
    let acks = 0;

    let result = irq_context
        .wait_on_irq(IRQ0, Duration::from_millis(1000))
        .unwrap();
    match result {
        irq::IrqWaitResult::TimedOut => println!("IRQ Timed Out"),
        irq::IrqWaitResult::IrqsAsserted(irqs) => {
            println!("IRQs Asserted: {:?}", irqs);
            assert!(irq_count_reg.read(&session).unwrap() == 0);
            session.acknowledge_irqs(IRQ0).unwrap();
            assert!(irq_count_reg.read(&session).unwrap() == 1);
        }
    }

    //session.close().unwrap();
}
