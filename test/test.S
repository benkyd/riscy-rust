ADDI x2, x0, 1

loop:
   SUB x1, x1, x2
   SW  x1, 4(x0) 
   BLT x0, x1, loop
