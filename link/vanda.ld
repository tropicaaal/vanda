OUTPUT_FORMAT("elf32-littlearm")
ENTRY(vector_table);

PROVIDE(__sys_stack_size = 0x800);
PROVIDE(__irq_stack_size = 0x800);
PROVIDE(__fiq_stack_size = 0x800);
PROVIDE(__abort_stack_size = 0x800);
PROVIDE(__svc_stack_size = 0x800);
PROVIDE(__undefined_stack_size = 0x800);

SECTIONS {
    . = 0;

    .text : {
        __image_start = .;
        *(.vectors)
        *(.text .text.*)
    }
    
    .rodata : {
        *(.rodata .rodata.*)
    }
    
    .data.rel.ro : {
        *(.data.rel.ro .data.rel.ro.*)
    }

    .got : {
        *(.got) *(.got.plt)
    }

    .data : {
        *(.data .data.*)
    }

    .rel.dyn : {
        __rel_start = .;
        *(.rel.dyn) *(.rel.*)
        __rel_end = .;
    }

    /*
     * These sections would normally be (NOLOAD) but aren't here, because we want
     * to reserve their space in the actual binary so that concatenation with the
     * kernel image is supported.
     */
    .bss : ALIGN(4) {
        __bss_start = .;
        *(.bss .bss.*)
        /* align the end of bss for zero loop cleanliness */
        . = ALIGN(8);
        __bss_end = .;
    }

    .stack : ALIGN(8) {
        . += __sys_stack_size;
        __sys_stack_top = .;

        . += __irq_stack_size;
        __irq_stack_top = .;

        . += __fiq_stack_size;
        __fiq_stack_top = .;

        . += __abort_stack_size;
        __abort_stack_top = .;

        . += __svc_stack_size;
        __svc_stack_top = .;

        . += __undefined_stack_size;
        __undefined_stack_top = .;
    }

    __payload_start = .;

    /DISCARD/ : {
        /* stuff in ARM's ELF spec that we don't need */
        *(.ARM.attributes*)
        *(.ARM.exidx*)
        *(.ARM.extab*)

        /* die */
        *(.comment*)

        /*
         * dynamic linker PIC plumbing that we don't need
         * TODO(tropicaaal): make sure that lld doesn't place these anyways
         */
        *(.hash .gnu.hash*)
        *(.dynamic*)
        *(.dynstr*)
        *(.dynsym*)
    }
}
/* 
ASSERT(SIZEOF(.dynamic) == 0, "ruh roh")
ASSERT(SIZEOF(.dynstr) == 0, "ruh roh")
ASSERT(SIZEOF(.dynsym) == 0, "ruh roh") */
