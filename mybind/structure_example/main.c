#include "structure_example.h"




// Functions that operate on the 'numbers' structure
int add(struct numbers *n) {
    return n->x + n->y;
}

int subtract(struct numbers *n) {
    return n->x - n->y;
}

int multiply(struct numbers *n) {
    return n->x * n->y;
}

int divide(struct numbers *n) {
    if (n->y == 0) {
        printf("Error: division by zero!\n");
        return 0;
    }
    return n->x / n->y;
}

// Static initialization of the 'number_functions' structure
static const struct number_functions num_funcs = {
    .numbers_add = add,
    .numbers_subtract = subtract,
    .numbers_multiply = multiply,
    .numbers_divide = divide,
};

int main() {
    struct numbers n = {10, 5};

    // Using the functions via the statically initialized structure
    printf("Addition: %d + %d = %d\n", n.x, n.y, num_funcs.numbers_add(&n));
    printf("Subtraction: %d - %d = %d\n", n.x, n.y, num_funcs.numbers_subtract(&n));
    printf("Multiplication: %d * %d = %d\n", n.x, n.y, num_funcs.numbers_multiply(&n));
    printf("Division: %d / %d = %d\n", n.x, n.y, num_funcs.numbers_divide(&n));

    return 0;
}
