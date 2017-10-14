Answers simu1 hw7
=======================

1. valgrind --tool=helgrind ./main-race

Helgrind does point at the right lines of code.

Helgrind shows us a "Possible data race during read of size 4 at 0x60207C by thread #1". Furthermore it gives us the information that "This conflicts with a previous write of size 4 by thread #2". It also shows "Address 0x60207c is 0 bytes inside data symbol "balance"".

This shows us the two competing threads that cause the date race and the variable in question.

2. valgrind --tool=helgrind ./main-race -v

2.1 There is no data race anymore since there arent two threads competing with each other. 

2.2 Helgrind observes a data race for each of the options and tells us that it found a "Lock at 0xFFEFF7A50 was first observed" (line 9 in main-race).

2.3 Helgrind still detects a data race between the two threads. This time it observes 2 locks (line 9 and 19 in main-race.c) instead of 1 as in 2.2.

3. ./main-deadlock

The program does not return an error message. Although looking at the code it seems like the locks from the second command can't be created right due to a wrong closure of the locks form the first command.

4. valgrind --tool=helgrind ./main-deadlock -v

Helgrind announces a problem in the thread of the second command. In this thread there is a lock order violation ("Thread #3: lock order "0x6020A0 before 0x6020E0" violated") meaning that the required order was violated, said order was established in the first thread.

5. valgrind --tool=helgrind ./main-deadlock-global -v

Looking at the code this program has the same problems as main-deadlock.c, although it has a global lock around the other locks the order still gets established and later violated. Helgrind should report the same issues as before. This tells us that Helgrind is able to access proccesses although they are locked.

6.

While the child is working the parent ends up doing nothing. Particularly if the child takes a long time to complete the parents resources are wasted.

7.

The code is correct, but inefficient as mentioned in answer 6. Helgrind reports a data race for the variable "done", because it gets accessed simultaneously by the two threads (read in the while loop, write in the child process).

8.

This version is preferred to the previous version. It is performance friendlier since it uses a signal-wait function to check for the condition variable instead of a while loop which checks the variable continuously and wastes CPU cycles. This version is also more correct due to the other version being error prone.

9. valgrind --tool=helgrind ./main-signal-cv -v

Helgrind reports no errors.
