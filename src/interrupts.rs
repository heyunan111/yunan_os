use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION : POINT \n {:#?}", stack_frame);
}

///“double fault” 异常 会 在执行主要（一层）异常处理函数时触发二层异常时触发。
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    //error code 不需要处理，因为他一直是0
    panic!(
        "EXCEPTION : DOUBLE FAULT \n {:#?} \n error code:{}",
        stack_frame, error_code
    );
}

pub fn init_idt() {
    IDT.load();
}

#[test_case]
fn test_exece_point() {
    x86_64::instructions::interrupts::int3();
}
