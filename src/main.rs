//! 🪻

#![no_std]
#![no_main]

use core::panic::PanicInfo;

core::arch::global_asm!(
    "
.section .vectors, \"ax\"
.global _vector_table

_vector_table:
    b _reset
"
);

#[unsafe(no_mangle)]
#[unsafe(naked)]
#[instruction_set(arm::a32)]
unsafe extern "aapcs" fn _reset() {
    core::arch::naked_asm!("blx _vanda_main");
}

struct A {
    p1: u16,
    p2: u8,
    p3: u32,
    s: &'static str,
}

static mut THING: A = A {
    p1: 0,
    p2: 0,
    p3: 0x03800000,
    s: "meow",
};

#[unsafe(no_mangle)]
unsafe extern "aapcs" fn _vanda_main() -> ! {
    loop {
        unsafe {
            core::ptr::write_volatile(&raw mut THING.p3, THING.p1 as u32);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
