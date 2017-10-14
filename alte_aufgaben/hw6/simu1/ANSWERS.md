ANSWERS HW6/simu1
======================

1. used flags: -j 3 -s (1-3) -c

Example for a lottery.py run and the computed solutions: python ./lottery.py -j 2 -s 3 -c
ARG jlist 
ARG jobs 2
ARG maxlen 10
ARG maxticket 100
ARG quantum 1
ARG seed 3

Here is the job list, with the run time of each job: 
  Job 0 ( length = 2, tickets = 54 )
  Job 1 ( length = 3, tickets = 60 )


** Solutions **

Random 625720 -> Winning ticket 88 (of 114) -> Run 1
  Jobs:  (  job:0 timeleft:2 tix:54 )   (* job:1 timeleft:3 tix:60 )  
Random 65528 -> Winning ticket 92 (of 114) -> Run 1
  Jobs:  (  job:0 timeleft:2 tix:54 )   (* job:1 timeleft:2 tix:60 )  
Random 13168 -> Winning ticket 58 (of 114) -> Run 1
  Jobs:  (  job:0 timeleft:2 tix:54 )   (* job:1 timeleft:1 tix:60 )  
--> JOB 1 DONE at time 3
Random 837469 -> Winning ticket 37 (of 54) -> Run 0
  Jobs:  (* job:0 timeleft:2 tix:54 )   (  job:1 timeleft:0 tix:--- )  
Random 259354 -> Winning ticket 46 (of 54) -> Run 0
  Jobs:  (* job:0 timeleft:1 tix:54 )   (  job:1 timeleft:0 tix:--- )  
--> JOB 0 DONE at time 5

2. used flags: -j 2 -l 10:1,10:100 -c

2.1

The probability of the process, with less tickets, of getting run is getting lower.

2.2

Since lottery scheduling is a probabilistic scheduling algorithm (taking the modulo of the available tickets from a randomly generated number) means there is never a non-zero probability for job 0 to not get run before job 1 completes. The probability is 1/101 times that job 0 is run when a scheduling operation is started. This desired distribution is more likely to get reached the longer the 2 jobs compete with each other.

2.3

Generally such a ticket imbalance leads to one process monopolizing the CPU.

3. used flags: -j 2 -l 100:100,100:100 -c

We calculate the unfairness metric U by dividing the time of the first job being completed by the time the second job is finished: 192/200 ~= 0.96.

used seeds: -s (0-4)

The unfairness metric is always higher or euqal than 0.95.

-s 0: 192/200 ~= 0.96
-s 1: 196/200 ~= 0.98
-s 2: 190/200 ~= 0.95
-s 3: 196/200 ~= 0.98
-s 4: 199/200 ~= 0.995

4. used flags: -j 2 -l 100:100,100:100 -s 0 -q (1-10) -c

Since the job length is a factor in the unfairness metric (the higher the job length the higher the unfairness metric) an increase of the quantum size of the job does reduce the job length thus leading to a lower unfairness metric.

5.

The graph is found in the HW6/simu1 folder in git with the name "Unfairness HW6.png". It shows the Unfairness metric with increasing job length. The longer the job length is the fairer the scheduling gets.
We always used the same ticket count for each job, our used inputs look like this:
1. ./lottery.py -j 2 -l 10:100,10:100 -c -s 0      => 0.75
2. ./lottery.py -j 2 -l 50:100,50:100 -c -s 0      => 0.94
3. ./lottery.py -j 2 -l 100:100,100:100 -c -s 0    => 0.96
4. ./lottery.py -j 2 -l 500:100,500:100 -c -s 0    => 0.969
5. ./lottery.py -j 2 -l 1000:100,1000:100 -c -s 0  => 0.971

Its also worth to change the ticket count because that also influences the unfairness. For example is the unfairness metric for ./lottery.py -j 2 -l 500:1000,500:1000 -c -s 0 => 932/1000 ~= 0.932 which is lower than the unfairness of 4. 0.969. All in all we can conclude that the unfairness gets fairer with increasing job length and gets worth with increasing ticket counts.

The stride scheduler is a deterministic fair-share scheduler since it takes the respective ticket counts of each job into account and calculates a stride value with it. Every job gets a stride counter which gets updated by the stride value after every time slice it ran. So overall the average run priority gets alot fairer by letting processes with less tickets run more often compared to the lottery scheduler. The problem is the implementation of a new process into a running system since the stride timer would need to be determined in a way that it wouldnt monopolize the CPU entirely until it caught up to the other processes stride counters(if the stride counter would be set to 0 when it enters). This means a graph with a stride scheduler would look different depending on the respective stride values. If the stride values are very close to each other the graph would look like our lottery scheduling graph but when the stride values would be very different some processes would get an unfair advantage and there reduce the overall fairness.
