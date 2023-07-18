addi    x2, x0, 0x69
lui     x3, 0xB00B5
bne     x2, x3, main
lui     x1, 0x420

main:
    addi x1, x0, 0x69
    lui x8, 0x80000
    lw x9, 0x0(x8)
    # addi x8, x8, 1
    # sw x8, 0x80000000
