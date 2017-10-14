1. Used flags: -l 3:0,5:100,5:100,5:100 -S SWITCH_ON_IO -I IO_RUN_LATER.

The total runtime of the processes are 27 time units. As in simu1 question 5 the first "5:100" is started before the IO process is finished. Due to IO_RUN_LATER all "5:100"'s are finished before returning to the 2 left over IO processes. The system resources are not effectively utilized since the CPU is idle during the last IO processes.

2. Used flags: -l 3:0,5:100,5:100,5:100 -S SWITCH_ON_IO -I IO_RUN_IMMEDIATE.

The runtime is significantly shorter, at 18 time units, since all the "5:100" processes are running simultaneously with the IO processes. Due to the "5:100" running simultaneously to the IO processes their values are not correct, this case will be corrected by running the processes again. Therefore it is a good idea to run a process after completing a IO again.

3. Used flags: -s 1 -l 3:50,3:50, -s 2 -l 3:50,3:50, -s 3 -l 3:50,3:50 with a combination of -I IO_RUN_IMMEDIATE, -I IO_RUN_LATER, -S SWITCH_ON_IO and -S SWITCH_ON_END.

When running IO_RUN_LATER or IO_RUN_IMMEDIATE nothing changes.

Seed 1:

The first seed has a 11 time unit runtime with IO, it uses the CPU 50% of the time and the IO 66.% of the time. With SWITCH_ON_END that switches to 42.86% for the CPU and 57.14% for the IO. The runtime increases to 14.

Seed 2:

Seed 2 has a 12 time unit runtime with the CPU being busy 46.15% and the IO being busy 84.62% of the time when running with SWITCH_ON_IO. If changed to SWITCH_ON_END the runtime jumps to 22 and the CPU is only busy 26.09% of the time, the IO busy-time goes down by ~15% to 69.75%.

Seed 3:

Seed 3 has a runtime of 13 time units. The CPU usage is at 46.15%, the IO usage at 69.23% when running SWITCH_ON_IO. The runtime increases to 18 time units when running with SWITCH_ON_END. The CPU usage shifts to 33.33% and the IO usage to 66.67%.

After all you can say that SWITCH_ON_IO works more efficient with having less runtime and better usage on the CPU and IO.
