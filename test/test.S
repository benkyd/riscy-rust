lui     x1, 0xFFFFF
addi    x2, x0, 0x123
addi    x2, x0, 0x123
lui     t1, 0x80000 # jalr main into x4 then jump to x4
jalr    t1, 0x4

main:
    addi t0, x0, 100
