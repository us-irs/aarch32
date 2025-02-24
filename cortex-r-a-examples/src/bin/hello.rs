//! Semihosting hello-world for Arm Cortex-R

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r_a_examples as _;

use semihosting::println;

cortex_r_a_examples::entry_point!();

/// The main function of our Rust application.
///
/// Called by [`kmain` or `boot_core`].
fn main() -> ! {
    let x = 1.0f64;
    let y = x * 2.0;
    println!("Hello, this is semihosting! x = {:0.3}, y = {:0.3}", x, y);
    panic!("I am an example panic");
}
