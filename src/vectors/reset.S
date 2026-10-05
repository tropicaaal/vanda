@ Reset vector

@ MARK: stack
@
@ our goal here it to setup a sane execution environment for Rust. the first
@ order of business is getting a usable stack in each procesor mode, which we
@ setup by switching to each mode and loading the stack.

.macro setup_mode_stack mode, label
    @ switch processor mode
    msr cpsr_c, #\mode
    
    @ load the stack
    @
    @ this is a little more complicated than the standard ldr-with-literal people use, since 
    @ have to offset it by our real physical load address and avoid ABS32 relocations which
    @ the linker will reject.
    @
    @ the method is similar to what the kernel decompressor does to load its own stack:
    @ <https://github.com/torvalds/linux/blob/master/arch/arm/boot/compressed/head.S#L1517>
    adr r0, \label @ address of label computed from pc
    ldr sp, [r0]   @ value of label (top of the stack - compile time addr of label)
    add sp, r0     @ add them together to cancel out label's address
.endm

@ NOTE: the mode bits we pass to cpsr_c also include CPSR.IFT, so IRQs and FIQs also get
@       masked here and I guess we also ensure we're in ARM state.
setup_mode_stack 0xDF, 0f @ sys
setup_mode_stack 0xD2, 1f @ irq
setup_mode_stack 0xD1, 2f @ fiq
setup_mode_stack 0xD7, 3f @ abort
setup_mode_stack 0xDB, 4f @ undefined
setup_mode_stack 0xD3, 5f @ svc

@ ... we are now running in supervisor mode, which is what the kernel wants at boot

@ MARK: relocations
@
@ Before Rust code runs, we need to adjust absolute pointers to static data (e.g. statics
@ referencing statics, extern statics, vtables, fat pointers, etc...) to point to the
@ correct address by adding vanda's real runtime base address to their offset.
@
@ The linker gives us a .rel.dyn section containing a table of dynamic relocations we need
@ to do. Our goal is to fixup all R_ARM_RELATIVE relocation Elf32_Rel entries.

@ compute __rel_start/__rel_end similar to how we did the stack
adr r0, 6f
ldr r1, [r0]
add r1, r0 @ r1 <- __rel_start

adr r0, 7f
ldr r2, [r0]
add r2, r0 @ r2 <- __rel_end

adr r0, __image_start @ r0 <- vanda's runtime load address

cmp r1, r2 @ __rel_start >= __rel_end?
bhs .Lrelocate_done @ bail out if the table is empty (or the start is somehow past the end)
.Lrelocate_entry:
    @ check the relocation entry type
    ldmia r1!, {{r3, r4}} @ r3 <- r_offset, r4 <- r_info; r1 += 8
    @ NOTE: this is kinda deceptively simple, since r4 is a bitfield that also
    @       encodes a symbol ID. Normally we'd want to mask out only the LSB with
    @       r4 & 0xFF, however we get away with it here because the symbol ID is
    @       always zero for R_ARM_RELATIVE, so we can just compare the whole word.
    cmp r4, #23 @ r4 == R_ARM_RELATIVE?

    @ apply R_ARM_RELATIVE fixup if needed
    ldreq r4, [r3, r0] @ r4 <- *(r_offset + base)
    addeq r4, r0       @ relocation += vanda_base
    streq r4, [r3, r0] @ *r4 = relocation

    cmp r1, r2           @ rel_ptr < rel_end?
    blo .Lrelocate_entry @ next entry

.Lrelocate_done:
    @ set VBAR to the load address (we place the vector table at __image_start+0x0).
    @ TODO(tropicaaal): check if VBAR is 32-byte aligned?
    mcr p15, 0, r0, c12, c0, 0

    @ MARK: bss clear
    adr r0, 8f
    ldr r1, [r0]
    add r1, r0 @ r1 <- __bss_start

    adr r0, 9f
    ldr r2, [r0]
    add r2, r0 @ r2 <- __bss_end

    movs r0, #0

    .Lclear_bss:
        cmp r1, r2
        beq .Lbss_done
        stm r1!, {{r0}}
        b .Lclear_bss
    .Lbss_done:
        @ MARK: entry
        b {entry} @ we're done, jump to rust main

@ relative offsets for computing label addresses above
.align 2
0: .word __sys_stack_top - .
1: .word __irq_stack_top - .
2: .word __fiq_stack_top - .
3: .word __abort_stack_top - .
4: .word __undefined_stack_top - .
5: .word __svc_stack_top - .

6: .word __rel_start - .
7: .word __rel_end   - .

8: .word __bss_start - .
9: .word __bss_end   - .
