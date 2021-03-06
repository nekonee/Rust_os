#![feature(panic_implementation)]
#![no_std]
#![no_main] 

use core::panic::PanicInfo;


#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    loop {}
}
