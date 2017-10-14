ANSWERS simu2
=============

1. used flags: -a 128 -p 512 -b 0 -l 20 -B 512 -L 20 -s 0

highest legal virtual address in segment 0: 0x00000013 (decimal: 19)

lowest legal virtual address in segment 1: 0x0000006C (decimal: 108)

highest illegal addresses in this entire address space: 0x0000006B (decimal: 107)

lowest illegal addresses in this entire address space: 0x00000014 (decimal: 20)

used flags to check: -a 128 -p 512 -b 0 -l 20 -B 512 -L 20 -s 0 -A 19,20,107,108 -c

using these flags runs the programm with the VAs 19,20,107 and 108. The results show that, as mentioned above, those are the highest and lowest illegal/legal addresses are.

2. used flags: -a 16 -p 128 -A 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15 --b0 ? --l0 ? --b1 ? --l1 ?

b0 = 0, l0 = 2, b1 = 16, l1 = 2
l has to be 2 because we only want the first and the last 2 VAs to be valid. Therefore we also have to set the bounds to the max and min (0 & 16)

3. used flags: -a 100 -l0 45 -l1 45

If we want 90% of the VAs to be valid we have to configure the simulation that (l0 + l1) >= a * 0.9, so 90% of the address space is inside either of the fragments. So the important values are the address space and the two limits.

4.

You can run the simulation so that no VAs are valid. You have to set the values for both of the limits to 0 so there is no space inside each of the segments therefore all the VAs have a segmentation violation.

5. 

  VA  0: 0x0000005c (decimal:   92) --> segmentation violation
  VA  1: 0x00000011 (decimal:   17) --> VALID in SEG0: 0x00000011 (decimal: 17)
  VA  2: 0x00000043 (decimal:   67) --> segmentation violation
  VA  3: 0x00000021 (decimal:   33) --> VALID in SEG0: 0x00000021 (decimal: 33)
  VA  4: 0x0000006c (decimal:  108) --> segmentation violation
  VA  5: 0x0000007a (decimal:  122) --> segmentation violation
  VA  6: 0x00000050 (decimal:   80) --> segmentation violation
  VA  7: 0x00000037 (decimal:   55) --> VALID in SEG0: 0x00000037 (decimal: 55)
  VA  8: 0x000000ff (decimal:  255) --> VALID in SEG1: 0x00000393 (decimal: 915)
  VA  9: 0x000000e9 (decimal:  233) --> segmentation violation
  VA 10: 0x00000001 (decimal:    1) --> VALID in SEG0: 0x00000001 (decimal: 1)
  VA 11: 0x0000014c (decimal:  332) --> VALID in SEG1: 0x000003E0 (decimal: 992)
  VA 12: 0x000000b4 (decimal:  180) --> segmentation violation
  VA 13: 0x000000cf (decimal:  207) --> segmentation violation
  VA 14: 0x0000012b (decimal:  299) --> VALID in SEG1: 0x000003BF (decimal: 959)
  VA 15: 0x00000084 (decimal:  132) --> segmentation violation

To answer this question we first checked if the VA was below 64 or above 235 (the limit for SEG1) to find out which VAs aren't a Segmentation Violation. The physical address for the first Segment were easy to get since they are the same as the VAs. For the other Segment we had to calculate the difference between the VA and the address space size only to subtract that value from the Segment 1 base (1024).
