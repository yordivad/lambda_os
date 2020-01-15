#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(lambda_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    lambda_os::println!("Lambda OS Initializing!");

    lambda_os::init();


    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo)  -> ! {
    lambda_os::println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lambda_os::test_panic_handler(info)
}

