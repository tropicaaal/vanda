//! 🪻

#![no_std]
#![no_main]

mod kboot;
mod vectors;

// on entry:
// - IRQs and FIQs are masked
// - Supervisor mode
// - ARM state
// - MMU, caches, peripherals are in an indeterminate state
unsafe extern "aapcs" fn vanda_main() -> ! {
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
