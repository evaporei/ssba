#include <time.h>
#include <stdio.h>

#define ITERS 1000000000

int main (int argc, char** argv) {
    clock_t start = clock();
    for (int i = 0; i < ITERS; i++);
    float secs = (float)(clock() - start) / CLOCKS_PER_SEC;
    float ops = ITERS / secs;
    printf("Elapsed: %.3f\n", secs);
    printf("Clock speed approx %.3f GHz\n", ops / 1000000000.0);
}
