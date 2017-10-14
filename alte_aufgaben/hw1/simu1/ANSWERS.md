ANSWERS
=============

The process-run.py script was run on a windows computer in the cmd with the distinct flags mentioned in the questions.

1. Used flags: -l 5:100, 5:100.

The command specifies that the CPU utilization is set to 100% and shall do 5 instructions.

2. Used flags: -l 1:0, 4:100.

It takes 9 time units to complete the task, 4 time units for the "4:100" command and 5 for the "1:0", as in the CPU is 5 busy for time units and the IO is busy for 4 time units at the end there is a done state of the length of 1 time unit.

3. Used flags: -l 1:0,4:100.

The processes take only 5 time units to finish. That is due to both of the commands running at the same time. The first time unit is allocated to the RUN:io and the CPU is in the "READY" state, after that the cpu is free to compute the "4:100" command.

4. Used flags: -l 1:0,4:100 -S SWITCH_ON_END.

This time the runtime is 9 time units long. The reason is that the "DONE" state of the IO command is reached at time unit 6 and the CPU is free to start the "4:100" command simultaneously. Compared to the runtime in question 2, there is no "DONE" state at the end.

5. Used flags: -l 1:0,4:100 -S SWITCH_ON_IO. 

Same runtime as in question 3, since the command SWITCH_ON_IO declares that the process shall not wait for the IO to finish before switching to the following process.
