#include "include/cpu.c"
#include "include/memory.c"
#include <stdio.h>
#include <stdlib.h>
int main(int argc, char const *argv[])
{
  if (argc < 2)
  {
    printf("usage: %s <filename>\n", argv[0]);
    return 1;
  }

  memory m;
  memory_init(&m);
  cpu c;
  cpu_init(&c);

  FILE *f = fopen(argv[1], "r");
  if (f == NULL)
  {
    printf("failed to open file: %s\n", argv[1]);
    return 1;
  }
  int16_t code[256];
  while (fscanf(f, "%hd", code) == 1)
  {
  }

  fclose(f);
  cpu_run(&c, &m);

  return 0;
}
