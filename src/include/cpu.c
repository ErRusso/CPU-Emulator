#ifndef CPU_C
#define CPU_C
#include <stdint.h>
#include "memory.c"
#include <printf.h>
typedef struct
{
  int16_t a, b, c, i;
  int16_t ip;
} cpu;

typedef void (*instruction)(cpu *c, memory *m);

void cpu_print_status(cpu *c, memory *m)
{
  printf("ip %d, code %d, [a %d,b %d,c %d, i %d]\n", c->ip, mem_get_code(m, c->ip), c->a, c->b, c->c, c->i);
}
void cpu_init(cpu *c)
{
  c->a = 0;
  c->b = 0;
  c->c = 0;
  c->i = 0;
  c->ip = 0;
}

void cpu_next(cpu *c, memory *m)
{
  c->ip += 1;
}

void cpu_halt(cpu *c, memory *m)
{
  c->ip = 255;
}

void cpu_run(cpu *c, memory *m)
{
  instruction instructions[256] = {
      &cpu_next,
      &cpu_halt,
  };

  while (c->ip < 256)
  {
    cpu_print_status(c, m);
    instructions[mem_get_code(m, c->ip)](c, m);
    c->ip += 1;
  }
}

#endif