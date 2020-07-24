
// Disables the std lib
#![no_std]
// Disables the normal main entrypoint
// With this added, a main function is not needed anymore
#![no_main]

// fn main() {}

// Handles Panics
// "!" indicates diverging function, which will not return
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


// eh_personality
// Language Items are special functions and types required internally by the compiler

// eh_personality marks a function that is used for implementing stack unwinding
// Requires liunwind od libs so need to remove by disabling unwinding
// Disabled by forcing abort on panic in Cargo.toml


// Start Attribute
// crt0 "C Runtime Zero" sets up the rust entrypoint before the main function is called


// Disables name mangling to output a function with the name _start otherwise it would be a funique name
#[no_mangle]

// esxtern "C" forces compiler to use C calling convention. _start is the default entry point name for most systems
// ! is because this is never returning/or called by a function, it is invoked directly by the OS
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

// Linker is a program that combines code into the executable
// Linker by default assues program depends on the C runtime by default. This is incorrect

// Need to set up rust to run in bare metal, aka embedded ARM system
// rustup target add thumbv7em-none-eabihf

// cargo build --target thumbv7em-none-eabihf

// 
extern crate rlibc;