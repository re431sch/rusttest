ANSWERS HW5 simu1
===========================

1. 

./scheduler.py -p FIFO -j 3 -c -l 200,200,200
  Job 0 ( length = 200.0 )
  Job 1 ( length = 200.0 )
  Job 2 ( length = 200.0 )

Execution trace:
  [ time   0 ] Run job 0 for 200.00 secs ( DONE at 200.00 )
  [ time 200 ] Run job 1 for 200.00 secs ( DONE at 400.00 )
  [ time 400 ] Run job 2 for 200.00 secs ( DONE at 600.00 )

Final statistics:
  Job   0 -- Response: 0.00  Turnaround 200.00  Wait 0.00
  Job   1 -- Response: 200.00  Turnaround 400.00  Wait 200.00
  Job   2 -- Response: 400.00  Turnaround 600.00  Wait 400.00

  Average -- Response: 200.00  Turnaround 400.00  Wait 200.00


./scheduler.py -p SJF -j 3 -c -l 200,200,200

  Job 0 ( length = 200.0 )
  Job 1 ( length = 200.0 )
  Job 2 ( length = 200.0 )

Execution trace:
  [ time   0 ] Run job 0 for 200.00 secs ( DONE at 200.00 )
  [ time 200 ] Run job 1 for 200.00 secs ( DONE at 400.00 )
  [ time 400 ] Run job 2 for 200.00 secs ( DONE at 600.00 )

Final statistics:
  Job   0 -- Response: 0.00  Turnaround 200.00  Wait 0.00
  Job   1 -- Response: 200.00  Turnaround 400.00  Wait 200.00
  Job   2 -- Response: 400.00  Turnaround 600.00  Wait 400.00

  Average -- Response: 200.00  Turnaround 400.00  Wait 200.00

SJF = shortest job first. Since all the jobs got the same length we see no difference compared to FIFO as expected.


2. 

./scheduler.py -p FIFO -j 3 -c -l 100,200,300
  Job 0 ( length = 100.0 )
  Job 1 ( length = 200.0 )
  Job 2 ( length = 300.0 )

Execution trace:
  [ time   0 ] Run job 0 for 100.00 secs ( DONE at 100.00 )
  [ time 100 ] Run job 1 for 200.00 secs ( DONE at 300.00 )
  [ time 300 ] Run job 2 for 300.00 secs ( DONE at 600.00 )

Final statistics:
  Job   0 -- Response: 0.00  Turnaround 100.00  Wait 0.00
  Job   1 -- Response: 100.00  Turnaround 300.00  Wait 100.00
  Job   2 -- Response: 300.00  Turnaround 600.00  Wait 300.00

  Average -- Response: 133.33  Turnaround 333.33  Wait 133.33

./scheduler.py -p SJF -j 3 -c -l 100,200,300
  Job 0 ( length = 100.0 )
  Job 1 ( length = 200.0 )
  Job 2 ( length = 300.0 )

Execution trace:
  [ time   0 ] Run job 0 for 100.00 secs ( DONE at 100.00 )
  [ time 100 ] Run job 1 for 200.00 secs ( DONE at 300.00 )
  [ time 300 ] Run job 2 for 300.00 secs ( DONE at 600.00 )

Final statistics:
  Job   0 -- Response: 0.00  Turnaround 100.00  Wait 0.00
  Job   1 -- Response: 100.00  Turnaround 300.00  Wait 100.00
  Job   2 -- Response: 300.00  Turnaround 600.00  Wait 300.00

  Average -- Response: 133.33  Turnaround 333.33  Wait 133.33

Again we expected no difference between FIFO and SJF since the Job0 is the shortest and first job and job1 is the 2nd shortest and 2nd job.

3. 

./scheduler.py -p RR -j 3 -c -l 200,200,200 -q 1
  [ time 589 ] Run job   1 for 1.00 secs
  [ time 590 ] Run job   2 for 1.00 secs
  [ time 591 ] Run job   0 for 1.00 secs
  [ time 592 ] Run job   1 for 1.00 secs
  [ time 593 ] Run job   2 for 1.00 secs
  [ time 594 ] Run job   0 for 1.00 secs
  [ time 595 ] Run job   1 for 1.00 secs
  [ time 596 ] Run job   2 for 1.00 secs
  [ time 597 ] Run job   0 for 1.00 secs ( DONE at 598.00 )
  [ time 598 ] Run job   1 for 1.00 secs ( DONE at 599.00 )
  [ time 599 ] Run job   2 for 1.00 secs ( DONE at 600.00 )

Final statistics:
  Job   0 -- Response: 0.00  Turnaround 598.00  Wait 398.00
  Job   1 -- Response: 1.00  Turnaround 599.00  Wait 399.00
  Job   2 -- Response: 2.00  Turnaround 600.00  Wait 400.00

  Average -- Response: 1.00  Turnaround 599.00  Wait 399.00


./scheduler.py -p RR -j 3 -c -l 100,200,300 -q 1
  [ time 591 ] Run job   2 for 1.00 secs
  [ time 592 ] Run job   2 for 1.00 secs
  [ time 593 ] Run job   2 for 1.00 secs
  [ time 594 ] Run job   2 for 1.00 secs
  [ time 595 ] Run job   2 for 1.00 secs
  [ time 596 ] Run job   2 for 1.00 secs
  [ time 597 ] Run job   2 for 1.00 secs
  [ time 598 ] Run job   2 for 1.00 secs
  [ time 599 ] Run job   2 for 1.00 secs ( DONE at 600.00 )

Final statistics:
  Job   0 -- Response: 0.00  Turnaround 298.00  Wait 198.00
  Job   1 -- Response: 1.00  Turnaround 499.00  Wait 299.00
  Job   2 -- Response: 2.00  Turnaround 600.00  Wait 300.00

  Average -- Response: 1.00  Turnaround 465.67  Wait 265.67

RR = Round Robin schedule. Every job runs for set timeslice (in our case 1 sec) and then the next job runs and so on and so on).

In the first task the jobs get finished more or less at the same time and in the 2 task job 0 is finished first and after that job 1 and job 2 finish in that order as seen in the final statistics table. This leads to a faster overall turnaround time since the cpu isnt "forced" to unncessary cycle through 3 jobs and instead only got 2 jobs or 1 job left to do.

4. 

Generally speaking FIFO and SJF lead to the same workloads when all jobs got the same length or when jobs with different length are sorted in an ascneding order. (for example 100,200,300 or 200,200,200)

5.

SJF and RR got the same workload when the jobs got the same length and the timeslice is set to the job length. An example would be 

./scheduler.py -p RR -j 3 -c -l 200,200,200 -q 200
AND
./scheduler.py -p SJF -j 3 -c -l 200,200,200

Furthermore if we only run 1 job they would have the same workload.

./scheduler.py -p RR -j 3 -c -l 1,1,100 -q 1 or -l 100,1,1 or -l 1,100,1
AND
./scheduler.py -p SJF -j 3 -c -l 100,1,1

In this example the workload is the same aswell since the RR schedule would run the first 2 jobs in the first 2 seconds (remember we got 1 second time slices) and then do the last job. The SJF schedule would do the 1sec job length jobs first aswell and then do the last job with joblength 100.

6.

if we increase the job length of at least 2 jobs (which is obviously needed with the SJF schedule) we expect an increase in the response time. Looking at the following examples shows that this is indeed the case:

./scheduler.py -p SJF -j 2 -c -l 100,1          Response Time = 0.50 sec   
./scheduler.py -p SJF -j 2 -c -l 200,1          Response Time = 0.50 sec    only increasing 1 job length doesnt make a difference at all since 1 job will be finished first and depending on that job length the Response time is calculated.
./scheduler.py -p SJF -j 2 -c -l 200,2          Response Time = 1.00 sec    increasing the job length of job1 does indeed increase the Response Time as expoected.
./scheduler.py -p SJF -j 3 -c -l 1000,500       Response Time = 250.00 sec

7.

the worst-case response time for the RR schedule can be calculated by the following equation:

Time slice * (number of jobs - 1) => q(N-1)
when the quantum length increases the response time increases aswell.
avarage response time: q(N-1) and time for last job to finish q(N-1)
