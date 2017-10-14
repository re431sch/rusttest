ANSWERS Task2/simu1
=====================

1. Used commands for compilation: gcc -g null.c, gcc null.c -o null

The programm compiles without errors, but when running the program its shows a "Segmentation fault".

The reason for this error is because we are trying to assign a non existing address to a variable.

2. Used commands: gdb null, "run".

Gdb shows us the path of the started programm at /home/mi431str/bsys-ss17-grp16/hw2/simu1/null and the process ID "13859" which is executing a new programm at /nix/store/w3zgdwkzb9n2h5n4hz17fn946sykkbjd-bash-4.4-p5/bin/bash".

3. Used command: valgrind --leak-check=yes ./null

Valgrind gives information about memory leaks. In this case the error message is "Invalid read of size 4 at 0x4006BA: main ( in /home/mi431str/bsys-ss17-grp16/hw2/simu1/null). Address 0x0 is not stack'd, malloc'd or (recently) free'd."
As already mentioned in Question 1 the address is not allocated.

4. Testprogram: malloc.c, same commands as in q1, q2 and q3.

Again the programm compiles without an error but this time it also runs without an error.

Gdb shows a similar result as in q2. The only difference is the PID "5081".

Valgrind shows there is a memory leak: "4 bytes in 1 blocks are definitely lost in loss record of 1 of 1 at "0x4C2ABBF: malloc (in /nix/store/fi1q3p8a1jzp56910h3ggf5mr6vz7gx0-valgrind-3.12.0/lib/valgrind/vgpreload_memcheck-amd64-linux.so) by 0x4006B7: main (malloc.c:6). That problem occurs because we are not freeing the allocated space at the end of the programm.

5. Testprogram: arrInt.c, same commands as in q1 and q3.

The program compiles properly and runs without an error. Valgrind shows the same errormessage as in question 3 because data[100] isnt in the allocated block of malloc (allocated 0-99) and there is the same error as in question 4 because the allocated space is not freed. The programm does not work correct, but it seems to work correctly. Since data[100] writes outside of the allocated space and maybe overwriting previously saved data it might lead to serious errors in the future.

6. Testprogramm: free.c, same commands as in q5.

Compilation and running works fine. Valgrind shows the same error as question 3 (invalid read of size 4), that is caused by trying access a allocated address which was already freed beforehand.

7. Testprogramm: free.c

The programm compiles with a warning (free.c:8:7: warning: passing argument 1 of \u2018free\u2019 makes pointer from integer without a cast [-Wint-conversion]free(data[50]);) but runs fine. There is no need for a new tool since valgrind is able to detect the source of the problem.