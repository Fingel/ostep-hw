#include <stdio.h>
#include <stdlib.h>
int main(int argc, char *argv[]) {
    // Intentionally write past the end of the allocated memory
    // Compiles and runs fine, which is wild. Valgrind reports an error.
    int *data = malloc(sizeof(int) * 100);
    data[100] = 1;
    printf("%d", data[100]);
    free(data);

}
