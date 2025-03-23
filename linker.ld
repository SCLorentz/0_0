OUTPUT_FORMAT(elf64-littleaarch64) /* ou OUTPUT_FORMAT(elf64-x86-64) para x86_64 */
ENTRY(_start)

SECTIONS {
    /* Define onde o código (.text) começa na memória */
    . = 0x100000;

    .text : {
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
    }

    /* Remove informações de depuração para reduzir tamanho */
    /DISCARD/ : {
        *(.comment)
        *(.note.*)
        *(.eh_frame)
    }
}