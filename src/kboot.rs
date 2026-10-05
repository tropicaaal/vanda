use core::arch::asm;

/// go do some light reading:
///
/// - <https://docs.kernel.org/arch/arm/booting.html>
/// - <https://www.cs.utexas.edu/~simon/378/resources/ARMv7-AR_TRM.pdf>
pub unsafe fn boot_kernel(kernel_addr: u32, dtb_addr: u32) -> ! {
    unsafe {
        log::debug!(
            "Hello from CPU{}! Preparing to boot kernel...",
            aarch32_cpu::asm::core_id()
        );

        // mask interrupts, wipe L1d
        asm!("cpsid if");
        aarch32_cpu::cache::clean_and_invalidate_l1_data_cache::<2, 5, 8>();
        asm!("dsb", "isb");
        log::debug!("L1d cleaned and invalidated.");

        // disable dcache and MMU, VEXos identity maps our execution space so we're fine to do this :3
        asm!(
            "mrc p15, 0, r0, c1, c0, 0",
            "bic r0, r0, #(1 << 2)", // SCTLR.C
            "bic r0, r0, #(1 << 0)", // SCTLR.M
            "mcr p15, 0, r0, c1, c0, 0",
            "dsb",
            "isb",
            out("r0") _
        );
        log::debug!("L1d and MMU disabled.");

        // invalidate icache and branch predictor
        asm!(
            "mov r0, #0",
            "mcr p15, 0, r0, c7, c5, 0", // invalidate icache
            "mcr p15, 0, r0, c7, c5, 6", // invalidate branch predictor array
            "dsb",
            "isb",
            out("r0") _
        );
        log::debug!("L1i and branch predictor array invalidated.");

        // invalidate TLBs
        asm!(
            "mov r0, #0",
            "mcr p15, 0, r0, c8, c7, 0", // TLBIALL
            "dsb",
            "isb",
            out("r0") _
        );
        log::debug!("TLBs invalidated.");

        log::debug!("Entering supervisor mode and jumping to kernel decompressor.");
        asm!(
            "mrs r0, cpsr",
            "bic r0, r0, #0b11111",
            "orr r0, r0, #0b10011",
            "msr cpsr_c, r0",
            // prepare registers for boot per ARM Linux boot protocol
            "mov r0, #0",          // r0 must be 0
            "mov r1, #0xFFFFFFFF", // r1 = MACH_TYPE (0xFFFFFFFF means DT-only rather than using ATAGs)
            // r2 = physical address of dtb
            // r3 = physical address of kernel
            "bx r3",
            options(noreturn),
            in("r2") dtb_addr,
            in("r3") kernel_addr,
        )
    }
}
