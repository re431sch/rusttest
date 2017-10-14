ANSWERS simu2 HW4
=====================

1. ./mem 1 and vmstat 1

When running 1 instance of ./mem 1 the usertime increases slightly by 10-20 percent in our case, in effect the idle time decreases. 
When running 2 instance of ./mem 1 the usertime increases even more by 20-30%. 
The increase in usertime makes sense since the CPU runs 1 or 2 tasks more.

2. ./mem 1024

When running ./mem 1024 for the first time the free memory instantly went down to ~0 (when we started we had approx 400 mb idle memory ready to be used) and the swpd column shows a increase to the maximum available memory. After killing the mem program the free memory went up to approc 1 GB and the used memory shows a slight decrease. That is expected because the program mem.c uses 1024 mb of memory. So it allocates that amount although it wasnt free at program start, when the program gets killed all the used memory from mem.c gets free'd and we see that in an increase in the free column of vmstat. Starting ./mem 1024 again shows the expected behaviour (only the free memory at the start is at roughly 1 gb this time): the free colums shows a dip to ~0 and when the program is killed it jumps up to ~ 1GB again.

3. ./mem (500/600/700/800/900/1000)

Before starting ./mem the first time for this question the "cat /proc/meminfo" command showed us a memtotal of 1048576 kB. According to the question we decided to run the mem.c program with values of 500/600/700/800/900/1000mb of allocated memory.
At the start vmstat shows us rougly 890mb free memory space. 

./mem 500: the si and so columns show increased activity, so some of the informtion gets swapped to and from the disc to the memory, but there isnt much of a difference to the state before mem.c gets run. Overall there are quite a few visible 0 values to be observed. 

./mem 600: As with 500 since the required memory space is lower than the actual system memory (memtotal) we observe the same slight increases in the so/si columns but nothing outstanding.

./mem 700: Same observations as 500 and 600.

./mem 800: Same observations as 500, 600 and 700.

./mem 900: since the allocated memory space of mem.c is higher than the actual available system memory at that time (as mentioned above it was roughly at 890 mb) we observe increased activity in the si/so columns for the first few loops but that dies down after some loops. As already observed in question 2 the mem.c program allocates more memory than available at the start but its not more than the memtotal. 

./mem 1000: since we allocate more memory than memtotal we see a huge increase in the swapping columns since the system needs to write the older data onto the disc and after that overwrites that data on the memory with new data. There is a difference between loop 0 and loop 1-xx since at that the time the memory is empty (loop 0) and after the next loop the old data gets overwritten.

4. ./mem (500/600/700/800/900/1000)

When running ./mem 1000 we see a huge increase in the bi/bo columns and we see a decrease in cpu user time but an increase in the time spent waiting for IO. As already explained in question 3 the allocated memory from mem.c is higher than the available system memory of our system so the system needs to swap files as blocks with the block device from the memory to the disc and back. That is why we see a huge increase in the bi/bo columns. Refering to the time spent waiting on the IO the amount calls is higher than the throughput of the block device which is why the CPU spends more time waiting for the IO.
When running with the others (./mem 500-900) only a slight increase is notable in in the bi/bo columns. The Time spent waiting on the IO stays the same.

5. used commands ./mem (500/600/700/800/900 or 1500) 


            ./mem 500  ./mem 600   ./mem 700   ./mem 800   ./mem 900   ./mem 1500
TIME    
loop0       429 ms      517 ms      691 ms      850 ms      820 ms      9830 ms      

loop 1-x    160 ms      200 ms      230 ms      250 ms      280 ms      36980 ms

BANDWIDTH
loop 0      1160        1160        1000        940         1100        150

loop 1-x    3100        3000        3100        3100        3100        40

The table shows that when the allocated memory of mem.c exceeds the maximum available systemmemory more swapping, more block transfers and more context switches are required. Therefore the bandwidth decreases which in turn leads to a much longer loop time.
When everything fits comfortably in the memeory the first loop takes longer and a lot more Bandwidth than the subsequent loops because the system needs to load all necessray data into the memory for the first time after that it doesnt need to do that.  When it doesn't fit the opposit is the case, the subsequent loops take 4 times as long as the first loop this is due to the memory being free for the first loop after that the system needs to overwrite the old data additionally.

The graph is included in the simu2 folder as "Q5.png".

6. 

/sbin/swapon shows us that we have ~3 GB (3145728000 bytes) swap space available.

we can exceed the availablpe swapß space up until ./mem 3100.
The reason we can go higher than 3 gb with mem.c is because the system doesnt use all the available space at once when swapping so it got a small margin (in our case 100 mb) which can go over the swap space limit. If we go over 3100 the program gets killed.
