ANSWERS simu 1
=================

1. Used flags: -s 0 -n 10 -l 930

you have to set the value of the limit register to 930 to get valid results for every address. That is because the highest address is at 929 and you have to add 1 due to the address at 0.

2. Used flags: -s 1 -n 10 -l 100

the maximal value is 14183 because the physical memory size is 16 kb. Calculation is as follows: 16384b (phys mem size) - 2201b (base) = 14183b.

3. Used flags:  -s 1 -n 10 -l (10 or 100) -a X -p Y (X = 2, 2048, 4096)(Y = 32, 8096, 32768)

Using variations of the X and Y variables showed that the base register is influenced by the value of the phys mem size, which also influences limit register value as already mentioned in question 2. The address space size changes the range the addresses can be in (when you set it to 2 the addresses can only be 1 or 0) and also the phys mem size always needs to be greater than the address mem size and the limit register.

4.

ARG phys mem size 16k

Base-and-Bounds register information:

  Base   : 0x00003952 (decimal 14674)
  Limit  : 1024

Virtual Address Trace
  VA  0: 0x0000024c (decimal:  588) --> VALID: 0x00003B9E (decimal: 15262)
  
  VA  1: 0x000004eb (decimal: 1259) --> SEGMENTATION VIOLATION
  
  VA  2: 0x000002bd (decimal:  701) --> VALID: 0x00003C0F (decimal: 15375)
  
  VA  3: 0x000000fa (decimal:  250) --> VALID: 0x00003A4C (decimal: 14924)
  
  VA  4: 0x00000486 (decimal: 1158) --> SEGMENTATION VIOLATION
  
  VA  5: 0x00000521 (decimal: 1313) --> SEGMENTATION VIOLATION
  
  VA  6: 0x0000065c (decimal: 1628) --> SEGMENTATION VIOLATION
  
  VA  7: 0x000001a3 (decimal:  419) --> VALID: 0x00003AF5 (decimal: 15093)
  
  VA  8: 0x0000063a (decimal: 1594) --> SEGMENTATION VIOLATION
  
  VA  9: 0x0000034d (decimal:  845) --> VALID: 0x00003C9F (decimal: 15519)
