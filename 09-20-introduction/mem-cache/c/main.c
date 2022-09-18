#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define SIZE 4000

void ij() {
  int i,j;
  static int x[SIZE][SIZE];
  for (i = 0; i < SIZE; i++) {
    for (j = 0; j < SIZE; j++) {
      x[j][i] = i + j;
    }
  }
}

void ji() {
  int i,j;
  static int x[SIZE][SIZE];
  for (j = 0; j < SIZE; j++) {
     for (i = 0; i < SIZE; i++) {
       x[j][i] = i + j;
     }
   }
}

int main() {
  clock_t start_ij = clock();
  ij();
  float secs_ij = (float)(clock() - start_ij) / CLOCKS_PER_SEC;
  printf("ij: %.3f\n", secs_ij);// 0.118

  clock_t start_ji = clock();
  ji();
  float secs_ji = (float)(clock() - start_ji) / CLOCKS_PER_SEC;
  printf("ji: %.3f\n", secs_ji);// 0.021
}
