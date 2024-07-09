#include "include/cpu.c"
#include "include/memory.c"
#include <printf.h>
int main(int argc, char const *argv[])
{
  memory m;
  cpu c;
  cpu_init(&c);

  int code[256] = {
      1,
  };
  for (int i = 0; i < 256; i++)
  {
    mem_put_code(&m, i, code[i]);
  }

  cpu_run(&c, &m);

  return 0;
}
