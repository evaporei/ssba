#include <stdio.h>

int main(int argc, char** argv) {
  unsigned long n = 1L << 63;
  printf("1 << 63 = %lu\n", n);// 9223372036854775808
  printf("1 << 64 = %lu\n", n << 1); // 0
}
