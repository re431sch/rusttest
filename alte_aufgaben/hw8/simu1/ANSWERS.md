#ANSWERS HW8
============

### main-two-cvs-while

0. 

The problem was trying to cast a pointer to an integer of a different size, for example can the pointer be 64 bit and the integer size can be 32 bit. We fixed it by using "uintptr_t" instead of "int".

1. used flags: -p 1 -c 1 -l 2 -m (1-2)

The increase of the buffer changes the use_ptr and fill_ptr values (since its modulo they get bigger aswell) in the code. The behaviour of the code stays the same.

2. used flags: -p 1 -c 1 -l 100 -m 10 -C 0,0,0,0,0,0,1

num_full goes to buffersize (10), then stays between buffersize and buffersize-1 before going back to 0.

3. 

not possible.

4. used flags: -p 1 -c 3 -l 10 -m 1 -v -C 0,0,0,1,0,0,0:0,0,0,1,0,0,0:0,0,0,1,0,0,0 -t

since we have 10 items from the producer the 3 Consumers need to wait at least 10 seconds at C3 and another 3 seconds to unlock all consumers at the end. So we expect a time of 13 seconds.

5. used flags: -p 1 -c 3 -l 10 -m 3 -v -C 0,0,0,1,0,0,0:0,0,0,1,0,0,0:0,0,0,1,0,0,0 -t

This time the code only runs for 12 seconds, because the last few processes don't get as staggered as before due to the programm being able to add all of the end-of-stream markers at the same time.

6. used flags: -p 1 -c 3 -l 10 -m 1 -v -C 0,0,0,0,0,0,1:0,0,0,0,0,0,1:0,0,0,0,0,0,1 -t

The one second pause at C6 only effects the time when the producer gets signaled to wake up and the buffer is not empty, because then the code switches back to a consumer to empty it, this totals in 2 seconds. Another 3 seconds get added at the end when unlocking the 3 consumers. Total time should be 5 seconds.

7. used flags: -p 1 -c 3 -l 10 -m 3 -v -C 0,0,0,0,0,0,1:0,0,0,0,0,0,1:0,0,0,0,0,0,1 -t

With those flags the time does not change. This is because the producer fills the buffer to max due to it having to wait for the consumers to finish their pause. Now that the buffer is full the consumers have to empty it one by one to add the end-of-stream markers, loosing the advantage over the smaller buffer.

### main-one-cv-while

1.

It is not possible to configure a sleep string in a way to cause problems for the code. That is due to the fact that we only got 1 producer and 1 consumer. When the producer sends the wakeup signal with the &cv parameter the only recipient is the sleeping consumer and vice versa. So no matter where we pause the code it doesnt cause problems.

2. used flags: -p 1 -c 2 -l 5 -m 1 -v -P 0,0,0,0,0,0,1

In this case with 1 producer and 2 consumers it is indeed possible to configure a sleep string to cause problems. A problem occurs when one of the consumers sends the wake up signal after emptying the buffer and the other consumer gets scheduled to start next, that consumer will then check whether the buffer is full and because the condition doesnt get met it will go sleeping again. This will result in every thread sleeping at the end. An example sleep string would look like this: -P 0,0,0,0,0,0,1

### main-two-cvs-if

1.

Again there is no possible sleep string to cause problems for the code when there is only 1 consumer and 1 producer, since a problem can only occur with for example 2 or more consumers. The reason is, that when the producer signals Consumer 1 that he can wake up, Consumer 2 can sneak in before and get the value from the buffer and thus leaving the buffer empty. When Consumer 1 finally tries to get a value from the buffer the code aborts with an error message: "tried to get an empty buffer". The problem lies in the if statement: Consumer 1 never rechecks the condition variable after getting the wake up call from the producer, instead he instantely tries to get a value from the buffer. A example sleep string would look like this: -C 0,1,0,0,0,0,0:0,1,0,0,0,0,0

### main-two-cvs-while-extra-unlock

1. 

The problem in releasing the lock before a put or get command is that another thread can access the buffer and change the value thus leading to problems. Lets assume the producer tries to fill a single entry buffer but releases the the lock. A consumer can sneak in and the code crashes because the buffer is empty or another producer sneaks in and fills the buffer. When the first producer tries to fill the buffer the code crashes aswell since the buffer is already full.


