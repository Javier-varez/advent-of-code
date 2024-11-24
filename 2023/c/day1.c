
#include <assert.h>
#include <fcntl.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define BUFFER_SIZE (2 * 1024 * 1024)

_Bool check_word(const char *const buffer, const size_t len,
                 const char *const w) {
  const size_t w_len = strlen(w);
  return strncmp(buffer, w, w_len < len ? w_len : len) == 0;
}

_Bool lookup_digit(const char *buffer, size_t len, size_t *digit) {
#define MATCH(str, val)                                                        \
  if (check_word(&buffer[0], len, str)) {                                      \
    *digit = val;                                                              \
    return 1;                                                                  \
  }

  switch (buffer[0]) {
  case '0' ... '9': {
    *digit = buffer[0] - '0';
    return 1;
  }
  case 'o': {
    MATCH("one", 1);
    break;
  }
  case 't': {
    MATCH("two", 2);
    MATCH("three", 3);
    break;
  }
  case 'f': {
    MATCH("four", 4);
    MATCH("five", 5);
    break;
  }
  case 's': {
    MATCH("six", 6);
    MATCH("seven", 7);
    break;
  }
  case 'e': {
    MATCH("eight", 8);
    break;
  }
  case 'n': {
    MATCH("nine", 9);
    break;
  }
  }
#undef MATCH

  return 0;
}

size_t find_pair(const char *buf, size_t len, size_t *first, size_t *last,
                 _Bool *valid) {
  _Bool found_first = 0;

  size_t offset = 0;
  while ((offset < len)) {
    const char c = buf[offset];

    if (c == '\n') {
      offset++;
      break;
    }

    size_t digit = 0;
    if (lookup_digit(&buf[offset], len - offset, &digit)) {
      if (!found_first) {
        *first = digit;
        found_first = 1;
      }
      *last = digit;
    }
    offset++;
  }

  *valid = found_first;
  return offset;
}

int main(void) {
  static char buffer[BUFFER_SIZE];
  int nread = read(0, buffer, BUFFER_SIZE);
  if (nread < 0) {
    printf("Error reading from standard input");
    exit(1);
  }

  assert(nread < BUFFER_SIZE);

  size_t sum = 0;
  size_t offset = 0;
  while (offset < (size_t)nread) {
    _Bool valid = 0;
    size_t first, last;
    const size_t consumed =
        find_pair(&buffer[offset], nread - offset, &first, &last, &valid);
    if (consumed == 0) {
      break;
    }

    offset += consumed;

    if (valid) {
      const size_t number = first * 10 + last;
      printf("number %lu\n", number);
      sum += number;
    }
  }
  printf("Sum is %lu\n", sum);
}
