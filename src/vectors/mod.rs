// <https://support.arm.com/documentation/dui0203/j/handling-processor-exceptions/armv6-and-earlier--armv7-a-and-armv7-r-profiles/the-vector-table>
core::arch::global_asm!(r#"
.section .vectors, "ax"
.align 5
.global vector_table

vector_table:
    b {reset}
    b {undefined_instruction}
    b {prefetch_abort}
    b {data_abort}
    nop
    b {irq}
    b {svc}
    b {fiq}
"#,
    reset = sym reset,
    undefined_instruction = sym undefined_instruction,
    prefetch_abort = sym prefetch_abort,
    data_abort = sym data_abort,
    irq = sym irq,
    svc = sym svc,
    fiq = sym fiq,
);

#[unsafe(naked)]
#[instruction_set(arm::a32)]
unsafe extern "C" fn reset() {
    core::arch::naked_asm!(
        include_str!("./reset.S"),
        entry = sym crate::vanda_main,
    )
}

#[unsafe(naked)]
#[instruction_set(arm::a32)]
unsafe extern "C" fn undefined_instruction() {
    core::arch::naked_asm!("b .")
}

#[unsafe(naked)]
#[instruction_set(arm::a32)]
unsafe extern "C" fn data_abort() {
    core::arch::naked_asm!("b .")
}

#[unsafe(naked)]
#[instruction_set(arm::a32)]
unsafe extern "C" fn prefetch_abort() {
    core::arch::naked_asm!("b .")
}

#[unsafe(naked)]
#[instruction_set(arm::a32)]
unsafe extern "C" fn irq() {
    core::arch::naked_asm!("b .")
}

#[unsafe(naked)]
#[instruction_set(arm::a32)]
unsafe extern "C" fn svc() {
    core::arch::naked_asm!("b .")
}

#[unsafe(naked)]
#[instruction_set(arm::a32)]
unsafe extern "C" fn fiq() {
    core::arch::naked_asm!("b .")
}
