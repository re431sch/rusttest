#include <stdio.h>
#include <stdlib.h>

int main()
{
	int *data;
	data = (int *) malloc(100 * sizeof(int));
	free(data[50]);	
	//printf("%d", data[50]);
	return 0;
}
