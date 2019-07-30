#ifndef Error_h
#define Error_h

#include <vector>
#include "./State.h"

void Error(string source, State &, u8 length, string message);

struct Error {
  std::vector<u16> lines;
  Error(string source, State &, u8 length, string message);
};

#endif
