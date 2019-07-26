#ifndef Variable_h
#define Variable_h

#include "./KSL.h"

#define RAW_DATA 0
#define PTR_SAFE 1
#define PTR_NULLABLE 2
#define PTR_UNSAFE 3

struct Variable
{
  string name;
  string type;
  u8 pointer;
  bool changeable;
  bool mutable;

  Variable(string name, string type, u8 pointer):
    name(name),
    type(type),
    pointer(pointer)
  {}
};

#endif
