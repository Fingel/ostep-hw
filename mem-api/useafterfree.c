#include <stdio.h>
#include <stdlib.h>
int main(int argc, char *argv[]) {
    // Intentionally free memory and then access it
    // Seems to compile and run fine, terrifying, but valgrind reports an error.
    int *data = malloc(sizeof(int) * 100);
    //free(data);
    // Second part of the question: this does generate a compiler error.
    // Also crashes at runtime.
    free(&data[50]);
    data[0] = 1;
    printf("%d", data[0]);

}
