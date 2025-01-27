#![feature(custom_test_frameworks)]
#![test_runner(yunan_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

use core::panic::PanicInfo;
use yunan_os::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    yunan_os::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("some sth");
}
