main:
  addi x3,x0,0       #  s = 0
  addi x4,x0,0       #  i = 0
  addi x5,x0,5       #  const 5
  lui  x6, 0x80000
  addi x6,x6,0x100   #  base address of ax[]
  addi x8,x0,0       #  offset = 0
loop:
  bge x4, x5, exit
  add x7, x6, x8    # compute effective address
  lw  x9, 0(x7)     # get ax[i]
  add x3, x3, x9    # s = s + ax[i]
  addi x8, x8, 4    # next element
  addi x4, x4, 1    # increment index
  j loop
exit:
