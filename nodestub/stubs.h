#include <stdlib.h>
#include <stdio.h>

#define NODESTUB(NAME) void NAME() { \
    fprintf(stderr, "============================\n"); \
    fprintf(stderr, "= fakenode: %s not fixed up!\n", # NAME); \
    fprintf(stderr, "= (crashing...)\n"); \
    fprintf(stderr, "============================\n"); \
    exit(77); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
    asm("nop"); \
}
