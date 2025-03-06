#include <stdio.h>
int main(int argc, char *argv[]){
    // Null pointer error. GDB and Valgrind report an error.
    int* x = NULL;
    int y = *x;
}
