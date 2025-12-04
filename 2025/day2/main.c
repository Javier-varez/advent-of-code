#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
  FILE *const input_file = fopen("input.txt", "r");

  size_t count = 0;
  char buffer[1024];

  while (fgets(buffer, sizeof(buffer), input_file)) {
    char *save = NULL;
    char *next = NULL;

    char *in_ptr = buffer;
    while ((next = strtok_r(in_ptr, ",", &save)) != NULL) {
      char *inner_save = NULL;
      char *begin = strtok_r(next, "-", &inner_save);
      assert(begin);
      char *end = strtok_r(NULL, "-", &inner_save);
      assert(end);

      const long b = strtol(begin, NULL, 10);
      const long e = strtol(end, NULL, 10);
      in_ptr = NULL;

      for (long i = b; i <= e; i++) {
        char data[1024];
        snprintf(data, sizeof(data), "%ld", i);
        const long l = strlen(data);

        if (l % 2 != 0) {
          continue;
        }

        count += (memcmp(&data[0], &data[l / 2], l / 2) == 0) ? i : 0;
      }
    }
  }

  printf("count %ld", count);
  return 0;
}
