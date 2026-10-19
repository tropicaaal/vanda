//! 🪻

#![no_std]
#![no_main]

// mod console;
mod boot;
mod cache;
mod payload;
mod vectors;

use payload::BootPlan;

// on entry from reset:
// - IRQs and FIQs are masked
// - Supervisor mode
// - ARM state
// - MMU, caches, peripherals are in an indeterminate state
unsafe extern "aapcs" fn vanda_main() -> ! {
    log::info!("====================================");
    log::info!(":: vanda 0.0.1 on CPU{} - meow :3 ::", aarch32_cpu::asm::core_id());
    log::info!("====================================");
    log::info!("");

    log::info!("Scanning for payloads...");

    match unsafe { BootPlan::scan() }.expect("could not locate payloads required for boot") {
        #[cfg(feature = "chainload")]
        BootPlan::Chainload { addr: _ } => {
            log::info!("Preparing to chainload...");
            todo!()
        },
        BootPlan::Kernel { kernel, device_tree } => {
            log::info!("Preparing to boot kernel...");
            unsafe { boot::boot_kernel(kernel.get(), device_tree.get()) }
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
