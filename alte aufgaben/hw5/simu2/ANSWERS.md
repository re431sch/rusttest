ANSWERS HW5 simu2
==========================

1. 

./mlfq.py -j 2 -n 2 -m 20 -M 0 -s 100 -c

Here is the list of inputs:
OPTIONS jobs 2
OPTIONS queues 2
OPTIONS quantum length for queue  1 is  10
OPTIONS quantum length for queue  0 is  10
OPTIONS boost 0
OPTIONS ioTime 5
OPTIONS stayAfterIO False
OPTIONS iobump False


For each job, three defining characteristics are given:
  startTime : at what time does the job enter the system
  runTime   : the total CPU time needed by the job to finish
  ioFreq    : every ioFreq time units, the job issues an I/O
              (the I/O takes ioTime units to complete)

Job List:
  Job  0: startTime   0 - runTime   5 - ioFreq   0
  Job  1: startTime   0 - runTime  23 - ioFreq   0


Execution Trace:

[ time 0 ] JOB BEGINS by JOB 0
[ time 0 ] JOB BEGINS by JOB 1
[ time 0 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 9 RUNTIME 5 TIMELEFT 4 ]
[ time 1 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 8 RUNTIME 5 TIMELEFT 3 ]
[ time 2 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 7 RUNTIME 5 TIMELEFT 2 ]
[ time 3 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 6 RUNTIME 5 TIMELEFT 1 ]
[ time 4 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 5 RUNTIME 5 TIMELEFT 0 ]
[ time 5 ] FINISHED JOB 0
[ time 5 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 9 RUNTIME 23 TIMELEFT 22 ]
[ time 6 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 8 RUNTIME 23 TIMELEFT 21 ]
[ time 7 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 7 RUNTIME 23 TIMELEFT 20 ]
[ time 8 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 6 RUNTIME 23 TIMELEFT 19 ]
[ time 9 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 5 RUNTIME 23 TIMELEFT 18 ]
[ time 10 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 4 RUNTIME 23 TIMELEFT 17 ]
[ time 11 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 3 RUNTIME 23 TIMELEFT 16 ]
[ time 12 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 2 RUNTIME 23 TIMELEFT 15 ]
[ time 13 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 1 RUNTIME 23 TIMELEFT 14 ]
[ time 14 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 0 RUNTIME 23 TIMELEFT 13 ]
[ time 15 ] Run JOB 1 at PRIORITY 0 [ TICKSLEFT 9 RUNTIME 23 TIMELEFT 12 ]
[ time 16 ] Run JOB 1 at PRIORITY 0 [ TICKSLEFT 8 RUNTIME 23 TIMELEFT 11 ]
[ time 17 ] Run JOB 1 at PRIORITY 0 [ TICKSLEFT 7 RUNTIME 23 TIMELEFT 10 ]
[ time 18 ] Run JOB 1 at PRIORITY 0 [ TICKSLEFT 6 RUNTIME 23 TIMELEFT 9 ]
[ time 19 ] Run JOB 1 at PRIORITY 0 [ TICKSLEFT 5 RUNTIME 23 TIMELEFT 8 ]
[ time 20 ] Run JOB 1 at PRIORITY 0 [ TICKSLEFT 4 RUNTIME 23 TIMELEFT 7 ]
[ time 21 ] Run JOB 1 at PRIORITY 0 [ TICKSLEFT 3 RUNTIME 23 TIMELEFT 6 ]
[ time 22 ] Run JOB 1 at PRIORITY 0 [ TICKSLEFT 2 RUNTIME 23 TIMELEFT 5 ]
[ time 23 ] Run JOB 1 at PRIORITY 0 [ TICKSLEFT 1 RUNTIME 23 TIMELEFT 4 ]
[ time 24 ] Run JOB 1 at PRIORITY 0 [ TICKSLEFT 0 RUNTIME 23 TIMELEFT 3 ]
[ time 25 ] Run JOB 1 at PRIORITY 0 [ TICKSLEFT 9 RUNTIME 23 TIMELEFT 2 ]
[ time 26 ] Run JOB 1 at PRIORITY 0 [ TICKSLEFT 8 RUNTIME 23 TIMELEFT 1 ]
[ time 27 ] Run JOB 1 at PRIORITY 0 [ TICKSLEFT 7 RUNTIME 23 TIMELEFT 0 ]
[ time 28 ] FINISHED JOB 1

Final statistics:
  Job  0: startTime   0 - response   0 - turnaround   5
  Job  1: startTime   0 - response   5 - turnaround  28

  Avg  1: startTime n/a - response 2.50 - turnaround 16.50


./mlfq.py -j 2 -n 2 -m 30 -M 0 -s 99 -c 


Here is the list of inputs:
OPTIONS jobs 2
OPTIONS queues 2
OPTIONS quantum length for queue  1 is  10
OPTIONS quantum length for queue  0 is  10
OPTIONS boost 0
OPTIONS ioTime 5
OPTIONS stayAfterIO False
OPTIONS iobump False


For each job, three defining characteristics are given:
  startTime : at what time does the job enter the system
  runTime   : the total CPU time needed by the job to finish
  ioFreq    : every ioFreq time units, the job issues an I/O
              (the I/O takes ioTime units to complete)

Job List:
  Job  0: startTime   0 - runTime  12 - ioFreq   0
  Job  1: startTime   0 - runTime   6 - ioFreq   0


Execution Trace:

[ time 0 ] JOB BEGINS by JOB 0
[ time 0 ] JOB BEGINS by JOB 1
[ time 0 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 9 RUNTIME 12 TIMELEFT 11 ]
[ time 1 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 8 RUNTIME 12 TIMELEFT 10 ]
[ time 2 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 7 RUNTIME 12 TIMELEFT 9 ]
[ time 3 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 6 RUNTIME 12 TIMELEFT 8 ]
[ time 4 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 5 RUNTIME 12 TIMELEFT 7 ]
[ time 5 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 4 RUNTIME 12 TIMELEFT 6 ]
[ time 6 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 3 RUNTIME 12 TIMELEFT 5 ]
[ time 7 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 2 RUNTIME 12 TIMELEFT 4 ]
[ time 8 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 1 RUNTIME 12 TIMELEFT 3 ]
[ time 9 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 0 RUNTIME 12 TIMELEFT 2 ]
[ time 10 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 9 RUNTIME 6 TIMELEFT 5 ]
[ time 11 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 8 RUNTIME 6 TIMELEFT 4 ]
[ time 12 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 7 RUNTIME 6 TIMELEFT 3 ]
[ time 13 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 6 RUNTIME 6 TIMELEFT 2 ]
[ time 14 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 5 RUNTIME 6 TIMELEFT 1 ]
[ time 15 ] Run JOB 1 at PRIORITY 1 [ TICKSLEFT 4 RUNTIME 6 TIMELEFT 0 ]
[ time 16 ] FINISHED JOB 1
[ time 16 ] Run JOB 0 at PRIORITY 0 [ TICKSLEFT 9 RUNTIME 12 TIMELEFT 1 ]
[ time 17 ] Run JOB 0 at PRIORITY 0 [ TICKSLEFT 8 RUNTIME 12 TIMELEFT 0 ]
[ time 18 ] FINISHED JOB 0

Final statistics:
  Job  0: startTime   0 - response   0 - turnaround  18
  Job  1: startTime   0 - response  10 - turnaround  16

  Avg  1: startTime n/a - response 5.00 - turnaround 17.00



2.

8.2: One job with a runtime of 200 and 3 queues => flasgs: -l 0,200,0 -Q 10,10,10 -c

8.3: Long running job with low priority that gets interupted by a short high priority job with a length of 20ms => flags: -l 0,180,0:100,20,0 -Q 10,10,10 -c

8.4: one long running job and one interactive job doing a lot of I/O => flags: -l 0,180,0:0,22,1 -Q 10,10,10 -i 9 -S -c

8.5 without Boost: One long running job and two interactive jobs that cause the long running job to starve => flags: -l 0,130,0:100,50,5:100,50,5 -Q 10,10,10 -i 5 -S -c

8.5 with Boost: adding a boost to prevent the starvation => flags: -l 0,130,0:100,50,5:100,50,5 -Q 10,10,10 -i 5 -S -B 50 -c

8.6 without gaming tolerance: One job can issue an I/O before a time slice ends and therefore dominate the CPU time => flags: -l 0,39,0;20,180,9 -Q 10,10,10 -i 1 -S -c

8.6 with gaming tolerance: Prevents one job from dominating the CPU time => flags: -l 0,130,0:20,70,9 -Q 10,10,10 -i 1 -c

8.7 two long running jobs run for 10ms in the highest queue, 20 in the middle and 40 in the lowest => flags: -l 0,100,0:0,100,0 -Q 10,20,40 -c

3. ./mlfq.py -l 0,100,0:0,100,0 -Q 10 -c

RR means that each job runs for a set time slice regardless how long the job length is. So if you set -Q to 10 you set the time each job runs to 10 it behaves like RR.

4. ./mlfq.py -l 0,25,0:20:60:19 -Q 20,20 -i 1 -S -c

Job 0 runs for 20 time slices, then job 1 starts and runs for 19 time slices after that it issues an I/O only to take control of the CPU again after it finishes the I/O

5. 

In this case you need to boost all the jobs every 200ms (10ms / 5% = 200ms) to the topmost queue again to prevent a job from starving.

6. 

The -i flag changed the amount of time slices an I/O should last. If a job is done with an I/O it's automatically set to the highest priority.

Example with -i 3:

[ time 6 ] Run JOB 0 at PRIORITY 2 [ TICKSLEFT 3 RUNTIME 14 TIMELEFT 7 ]
[ time 7 ] Run JOB 0 at PRIORITY 2 [ TICKSLEFT 2 RUNTIME 14 TIMELEFT 6 ]
[ time 8 ] IO_START by JOB 0
IO DONE
[ time 8 ] Run JOB 1 at PRIORITY 2 [ TICKSLEFT 9 RUNTIME 76 TIMELEFT 75 ]
[ time 9 ] Run JOB 1 at PRIORITY 2 [ TICKSLEFT 8 RUNTIME 76 TIMELEFT 74 ]
[ time 10 ] Run JOB 1 at PRIORITY 2 [ TICKSLEFT 7 RUNTIME 76 TIMELEFT 73 ]
[ time 11 ] IO_START by JOB 1
IO DONE
[ time 11 ] IO_DONE by JOB 0
[ time 11 ] Run JOB 2 at PRIORITY 2 [ TICKSLEFT 9 RUNTIME 50 TIMELEFT 49 ]
[ time 12 ] Run JOB 2 at PRIORITY 2 [ TICKSLEFT 8 RUNTIME 50 TIMELEFT 48 ]
[ time 13 ] Run JOB 2 at PRIORITY 2 [ TICKSLEFT 7 RUNTIME 50 TIMELEFT 47 ]
[ time 14 ] IO_DONE by JOB 1
[ time 14 ] Run JOB 2 at PRIORITY 2 [ TICKSLEFT 6 RUNTIME 50 TIMELEFT 46 ]
[ time 15 ] Run JOB 2 at PRIORITY 2 [ TICKSLEFT 5 RUNTIME 50 TIMELEFT 45 ]
[ time 16 ] IO_START by JOB 2
IO DONE
[ time 16 ] Run JOB 0 at PRIORITY 2 [ TICKSLEFT 9 RUNTIME 14 TIMELEFT 5 ]
[ time 17 ] Run JOB 0 at PRIORITY 2 [ TICKSLEFT 8 RUNTIME 14 TIMELEFT 4 ]
[ time 18 ] Run JOB 0 at PRIORITY 2 [ TICKSLEFT 7 RUNTIME 14 TIMELEFT 3 ]
[ time 19 ] IO_DONE by JOB 2
[ time 19 ] Run JOB 0 at PRIORITY 2 [ TICKSLEFT 6 RUNTIME 14 TIMELEFT 2 ]
[ time 20 ] Run JOB 0 at PRIORITY 2 [ TICKSLEFT 5 RUNTIME 14 TIMELEFT 1 ]

