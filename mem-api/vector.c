#include <stdio.h>
#include <stdlib.h>

struct vector {
    int* data;
    size_t size;
    size_t capacity;
};

int init(struct vector* v, size_t capacity) {
    v->data = malloc(capacity * sizeof(int));
    if (!v->data) return -1;
    v->size = 0;
    v->capacity = capacity;
    return 0;
}

int grow(struct vector* v) {
    size_t new_capacity = v->capacity * 2;
    int* new_data = realloc(v->data, new_capacity * sizeof(int));
    if (!new_data) return -1;
    v->data = new_data;
    v->capacity = new_capacity;
    return 0;
}

int push(struct vector* v, int value) {
    if (v->size == v->capacity) {
        grow(v);
        return push(v, value);
    } else {
        v->data[v->size++] = value;
        return 0;
    }
}

void print_vector(struct vector* v) {
    printf("Capacity: %zu - ", v->capacity);
    printf("[");
    for (size_t i = 0; i < v->size; i++) {
        printf("%d", v->data[i]);
        if (i < v->size - 1) printf(", ");
    }
    printf("]\n");
}

int main() {
    struct vector array;
    init(&array, 2);
    print_vector(&array);
    push(&array, 2);
    print_vector(&array);
    push(&array, 3);
    push(&array, 4);
    push(&array, 5);
    print_vector(&array);
    push(&array, 6);
    print_vector(&array);
    free(array.data);
}
