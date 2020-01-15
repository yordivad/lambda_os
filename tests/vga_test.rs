#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(lambda_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    lambda_os::serial_println!("vga testing...");
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lambda_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
    lambda_os::serial_println!("[ok]");
}