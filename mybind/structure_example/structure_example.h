#include <stdio.h>

// Definition of the 'Numbers' structure
struct Numbers {
    int x;
    int y;
};

// Definition of the 'number_functions' structure
struct Number_Functions {
    int (*numbers_add)(struct Numbers *n);
    int (*numbers_subtract)(struct Numbers *n);
    int (*numbers_multiply)(struct Numbers *n);
    int (*numbers_divide)(struct Numbers *n);
};