//! SVC (software interrupt) example for Arm Cortex-R

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r_a_examples as _;

use semihosting::println;

cortex_r_a_examples::entry_point!();

/// The main function of our Rust application.
///
/// Called by [`kmain`].
pub fn main() -> ! {
    let x = 1;
    let y = x + 1;
    let z = (y as f64) * 1.5;
    println!("x = {}, y = {}, z = {:0.3}", x, y, z);
    cortex_r_a::svc!(0xABCDEF);
    println!("x = {}, y = {}, z = {:0.3}", x, y, z);
    panic!("I am an example panic");
}

/// This is our SVC exception handler
#[no_mangle]
unsafe extern "C" fn _svc_handler(arg: u32) {
    println!("In _svc_handler, with arg={:#06x}", arg);
    if arg == 0xABCDEF {
        // test nested SVC calls
        cortex_r_a::svc!(0x456789);
    }
}
