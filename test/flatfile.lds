ENTRY(_start)
SECTIONS {
  .text 0x80000000 : {
      *(.start);
      *(.text);
      *(.text.*)
      }
  .bss : { *(.bss); *(.bss.*) }
  .data : { *(.data); *(.data.*) }
  .rodata : { *(.rodata); *(.rodata.*) }
}
