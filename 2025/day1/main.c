#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
  FILE *const input_file = fopen("input.txt", "r");

  long position = 50;
  long count = 0;

  char buffer[32];
  while (fgets(buffer, sizeof(buffer), input_file)) {
    bool negative = false;
    if (buffer[0] == 'L') {
      negative = true;
    }

    long n = strtol(&buffer[1], NULL, 10);
    n = n % 100;
    if (negative) {
      n = -n;
    }

    position += n;
    position = position % 100;

    count += position == 0;
  }

  printf("count %ld", count);
  return 0;
}
