#ifndef MEMORY_C
#define MEMORY_C
typedef struct
{
  int data[256];
  int code[256];
} memory;

void memory_init(memory *m)
{
  for (int i = 0; i < 256; i++)
  {
    m->data[i] = 0;
    m->code[i] = 0;
  }
}

void mem_put_data(memory *m, int address, int value)
{
  m->data[address] = value;
}

void mem_put_code(memory *m, int address, int value)
{
  m->code[address] = value;
}

int mem_get_data(memory *m, int address)
{
  return m->data[address];
}

int mem_get_code(memory *m, int address)
{
  return m->code[address];
}
#endif