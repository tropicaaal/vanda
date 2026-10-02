//! 🪻

#![no_std]
#![no_main]

mod kboot;
mod vectors;

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

unsafe extern "aapcs" fn vanda_main() -> ! {
    loop {
        unsafe {
            core::ptr::write_volatile(&raw mut THING.p3, THING.p1 as u32);
        }
    }

    log::debug!("===========================");
    log::debug!(":: vanda 0.0.1 - meow :3 ::");
    log::debug!("===========================");
    log::debug!("");

    unsafe { kboot::boot_kernel(0x5000000, 0x7E00000) }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
