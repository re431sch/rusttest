ANSWERS simu 3
================

1. used flags: -n 10 -H 0 -p BEST/WORST -s 0

The BEST fit takes less space than the WORST fit (16 Byte vs 33 Byte used). This is because the BEST fit uses the already used and free'd space again, for example 3 Bytes allocated and free'd, the BEST fit uses those 3 Bytes again to allocate 2 Bytes later, the WORST fit doesn't.

2. used flags: -n 10 -H 0 -p BEST/FIRST -s 0

In this case only the "searched elements" change. Since the first possible space is the best. FIRST fit is faster if you allocate and free a sequence of Sizes (1,2,3,4 -> 1,2,3,4) a few times since BEST checks all the possible spaces and compares them only to take the first one anyway and FIRST fit just takes the first possible one which would be the best in that scenario.

3. used flags: -n 10 -H 0 -p BEST/FIRST/WORST -s 0 -l ADDRSORT/SIZESORT+/SIZESORT-

Nothing changes with the BEST fit and any of the sorts since BEST always checks every entry in the list so it doesnt matter how they are sorted. The same can be said about the WORST fit since this one always takes the biggest space and has to search the whole list to find it anyway. The only fit that changes is the FIRST fit since it takes the first chunk that is big enough. So if you use FIRST fit and a SIZESORT+ you have a faster BEST fit since the smallest chunk is the first one and FIRST fit will take the first one that is big enough without checking the rest of the list.

4. used flags: -n 1000 -H 0 -p BEST/FIRST/WORST -s 0 -l ADDRSORT/SIZESORT+/SIZESORT- -C

The coalescing only works with the ADDRSORT since it cant merge if the addresses aren't nearby and only the ADDRSORT sorts the addresses. Therefore coalescing doesn't work with either of the SIZESORTs since they don't care for the addresses and only sort by size. Due to this you can't use coalescing on those lists.

Example of the Free List after the last operation with BEST and ADDRSORT: 

ptr[514] = Alloc(2)  returned 1000 (searched 1 elements)
Free List [ Size 1 ]:  [ addr:1002 sz:98 ] 

Example of the Free List after the last operation with BEST and SIZESORT+:

ptr[583] = Alloc(4)  returned 1076 (searched 29 elements)
Free List [ Size 28 ]:  [ addr:1090 sz:1 ] [ addr:1038 sz:1 ] [ addr:1029 sz:1 ] [ addr:1027 sz:1 ] [ addr:1036 sz:1 ] [ addr:1060 sz:1 ] [ addr:1028 sz:1 ] [ addr:1037 sz:1 ] [ addr:1005 sz:1 ] [ addr:1080 sz:1 ] [ addr:1075 sz:1 ] [ addr:1004 sz:1 ] [ addr:1019 sz:1 ] [ addr:1094 sz:2 ] [ addr:1068 sz:2 ] [ addr:1011 sz:2 ] [ addr:1030 sz:3 ] [ addr:1050 sz:3 ] [ addr:1091 sz:3 ] [ addr:1081 sz:3 ] [ addr:1033 sz:3 ] [ addr:1045 sz:5 ] [ addr:1070 sz:5 ] [ addr:1006 sz:5 ] [ addr:1084 sz:6 ] [ addr:1020 sz:7 ] [ addr:1061 sz:7 ] [ addr:1053 sz:7 ] 


5. used flags: -n 10 -H 0 -p BEST -s 0 -P (0-100)

"-P" changes the percent of operations that are allocs. So with the standard 50% half of the Operations are allocs and the other half frees. Changing that value to higher numbers the program will still run fine but in none of the allocated space gets free'd. If you set the value to lower than 50% every space that gets allocated will be free'd it has to be the 50:50 on the different operations since you can't free space you haven't allocated yet. If you set the Value to 0 you will get a assertion error since the programm doesn't allow a value smaller than 1.

Example of the Free List after the last operation with -n 10 -H 0 -p BEST -s 0 -P 99

ptr[9] = Alloc(10)  returned 1053 (searched 1 elements)
Free List [ Size 1 ]:  [ addr:1063 sz:37 ] 

As you can see there are only 37 byte left at the end of the operations, because none of the allocated space gets free'd.

Example of the Free List after the last operation with -n 10 -H 0 -p BEST -s 0 -P 0

Traceback (most recent call last):
  File "malloc.py", line 162, in <module>
    assert(percent > 0)
AssertionError

6. used flags: -H 0 -p BEST/FIRST/WORST -A x

To generate a highly framgented free space the best fit to use is WORST since it always takes the biggest chunk to allocate new space. o further fragment the space choose very short spaces to allocate. 
