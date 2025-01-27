#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(yunan_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use yunan_os::interrupts::init_idt;

mod serial;
mod vga_buffer;

#[no_mangle] // 不重整函数名
             // 因为链接器会寻找一个名为 `_start` 的函数，所以这个函数就是入口点
             // 默认命名为 `_start`
pub extern "C" fn _start() -> ! {
    init_idt();

    #[cfg(test)]
    test_main();

    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    yunan_os::test_panic_handler(info)
}
