OUTPUT_FORMAT("elf32-littlearm")
ENTRY(_vector_table);

SECTIONS {
    .text : {
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
    .bss : {
        __bss_start = .;
        *(.bss .bss.*)
        /* align the end of bss for zero loop cleanliness */
        . = ALIGN(8);
        __bss_end = .;
    }

    .stack : ALIGN(8) {
        __stack_bottom = .;
        . += 0x4000;
        __stack_top = .;
    }

    __image_end = .;

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