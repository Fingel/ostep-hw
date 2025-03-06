#include <stdio.h>
#include <stdlib.h>
int main(int argc, char *argv[]) {
    // Does not free memory allocated by malloc.
    // Valgrind reports an error.
    int* x = malloc(42);
    printf("Goodbye!");
}
