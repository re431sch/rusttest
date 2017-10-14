ANSWERS simu1
==============

1. used flags: -P (1k/2k/4k) -a (1m/2m/4m) -p 512m -v -n 0

The page table size can be changed by adjusting either the address space size (-a flag) or the page size (-P flag). This is because if the address space grows there is space for more pages, likewise if the page size gets smaller you can fit more pages in the same address space. If the pages get to big we either have a too big address space size, which takes a lot of memory space, or we have just a small number of pages, which leads to higher run times, due to a higher miss rate. For example if you set the Page size to 1k and the address space size to 4m the page table size is 4096 (4096k address space size /1k page size) and if you set them to 4k and 1m you get a page table size of 256.

2. used flags: -P 1k -a 16k -p 32k -v -u (0/25/50/75/100)

The higher you set the percentages the more pages in the address get their valid bits set to valid, therefore more of the VAs are valid in the end. Remarkable is that due to the small sample size all of the pages are valid at only 75%.

3. 

-P 1m -a 256m -p 512m -v -s 1: This is unrealistic due to the page table size being too big (256m/1m = 256)which in turn leads to a higher memory usage and a high chance of TLB misses.


4.

The address space must be smaller than the physical memory size (Error: physical memory size must be GREATER than address space size (for this simulation) = this obviously is the case since the OS cant reserve a greater address space size than physical memory space is directly available from the hardware itself.) and the physical mem size has to be less than 1 GB (Error: must use smaller sizes (less than 1 GB) for this simulation) and bigger than 0 (must specify a non-zero physical memory size). Furthermore the address space cant be 0 or lower than the pagesize or it will return an Error ("IndexError: index out of range").
