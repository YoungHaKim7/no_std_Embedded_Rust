#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

// #![feature(lang_items)]
// #[lang = "eh_personality"]
extern "C" fn eh_personality() {}

use arrayvec::ArrayString;
use numtoa::NumToA;

fn main() {
    let mut num_buffer = [0u8; 20];
    let mut text = ArrayString::<100>::new();

    let num1 = 123;
    let num2 = 456;
    let num3 = 789;

    // text.clear(); (on subsequent usages)
    text.push_str("example ");
    text.push_str(num1.numtoa_str(10, &mut num_buffer));

    text.push_str(" test ");
    text.push_str(num2.numtoa_str(10, &mut num_buffer));

    text.push_str(" words ");
    text.push_str(num3.numtoa_str(10, &mut num_buffer));
}
