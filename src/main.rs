#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;

use blog_os::println;

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    println!("Hello World{}", "!");

    blog_os::init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    loop {}
}


#[test_case]
fn trivial_assertion() {
    // serial_println!("trivial assertion...");
    assert_eq!(1, 1);
    // serial_println!("[ok]");
}
