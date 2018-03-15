/**
 * Simple example C program which calls through to a function defined in Rust.
 * The two command line arguments are compared using an implementation of
 * Jaro-Winkler.
 */

#include <stdio.h>

// Import from Rust library.
double jaro_winkler(char* a, char* b);

int main(int argc, char** argv) {
  char* prog_name = argc < 1 ? "this_program" : argv[0];
  if (argc != 3) {
    printf("Usage: %s <string1> <string2>\n", prog_name);
    return 1;
  }

  char* str_a = argv[1];
  char* str_b = argv[2];
  double dist = jaro_winkler(str_a, str_b);

  printf("Jaro-Winkler similarity of '%s' and '%s' is %f.\n",
      str_a, str_b, dist);

  return 0;
}
