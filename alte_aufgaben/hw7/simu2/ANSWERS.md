ANSWERS HW7
==============


# flags.s

1.1 -p flag.s

The program checks whether the lock is free and if it is increments count by 1. 

1.2 -p flag.s

If you run the program with the default parameters it works as expected.

1.3 -p flag.s -c -R ax,bx -M 100,104

The programm computes the correct results. Since there are 2 threads running count should be at 2 and it is.

1.4 

Flag should have the value 0 since the lock should be released as seen in line 19 from flag.s.

2. -p flag.s -c -R ax,bx -M 100,104 -a bx=2,bx=2

The count value goes up to 4 since every thread runs the program twice. Flag is still at 0 since the program resets it every time.

3. -p flag.s -c -R ax,bx -M 100,104 -a bx=4,bx=4 -i (1-99)

If the interrupt frequency goes below 8 the count value does not end up correct. That is due to the lock being released after 8 commands and if it interrupts before that happens, the other thread is not able to write a new value into count.

# test-and-set.s

1. -p test-and-set.s -c -R ax,bx -M 100,104 -i 1 -a bx=4,bx=4

If we set the interrupt frequency to 1 the code works but is inefficient, that is due to the fact that one thread waits for the other to finish (unlock) and in the meantime wastes CPU-cycles. Effectively the code lines 8-11 are continously looped through until the lock is released.

2. -p test-and-set.s -c -R ax,bx -M 100,104 -a bx=4,bx=4 -P 00000111111111111111

The program still works as intended when grabbing the lock in one thread and then trying to grab it in the other one. Even trying a random combination of program scheduling does not affect the correct outcome.

# peterson.s

1. -p peterson.s -c -R ax,bx,cx -M 100,108,112 -i 2 (example failure frequency)

Whenever the code is run with an interrupt in the critical section (line 42-44) the count value gets not computed correctly due to ax getting moved to count and not added on top of the old value.

2. p peterson.s -c -R ax,bx,cx -M 100,108,112 -P 1100

The P flag lets use define the program scheduling in a way that the program runs flawlessly by making sure that both threads dont enter their critical section at the same time. Obvioulsy we can make the program not work correctly when we choose the aforementioned interrupt frequencies (an example being "-P 1100" like "-i 2").

# ticket.s

1. -p ticket.s -c -R ax,bx,cx,dx -a bx=1000,bx=1000

Yes there is a lot of spinning wait time, which is shown by the amount of extra commands the second thread needs to finish its 1000 iterations of the code.

2. -p ticket.s -c -R ax,bx,cx,dx -a bx=1000,bx=1000,bx=1000,bx=1000 -t 4

Adding more threads, you can clearly see that the first thread is always finished much earlier than the other threads.

#yield.s

1. It correlates directly with the amount of threads running. If we got 10 threads running and 1 of those aquired the lock there would happen 9 run and yield patterns whereas the test-and-set would waste 9 time slices spinning.

#test-and-test-and-set.s

1.

Works just like the test-and-set.s programm but line 8-10 make sure that %ax is 0 all the time even if other threads happen to change the value.

2.

Due to the aforementioned optimized code the code itself is less error prone. 
