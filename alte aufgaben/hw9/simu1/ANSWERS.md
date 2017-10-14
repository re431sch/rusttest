ANSWERS HW9 simu1
==============================

1.  used flags: -n 2 -l 1 -v

when running the programm a few times it does indeed happen, that thread 1 will run most of the time first since it aquires the lock first. From time to time we can see that thread 0 runs because it still runs when when thread 1 still is in its critical section thus aquiring the lock first and finish before thead 1 is able to run.

2.

No the program didnt deadlock until a very high number of loops. At 1000 for example the programm deadlocks completely so that only one of the threads is running.

3.

as already stated in question 2 increasing the number of loops increases the chance of deadlock...increasing the number of threads also increases the chance of a deadlock since there are more thread competing for the lock.

That means by increasing both the loop count and the thread count we also drastically increase the chance of a deadlock.

4.

By the if statement in the code we can see that the programm always will give the lock to the smaller vector first , so even with the -d flag enabled (which switches the sequence in which the vector call *vector_add(v2, v1)* and *vector_add(v1, v1)*) it still gives the lock to the smaller vector nontheless.

The statement where the vectors are the same helps in avoiding a deadlock by giving the lock to src (and dst since they are the same...).

5. 


