#include <stdio.h>

int main (void)
{
    //int x = 10;
    //x = NULL; 
    int *a = NULL;
    int b = *a;
    printf("%d", b);
    return 0;
}
