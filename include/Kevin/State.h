#ifndef State_h
#define State_h

#include <unordered_map>
#include "./KSL.h"

struct State
{
  u32 offset;
  string expected_return;
  std::unordered_map<string, string> variables;

  State():
    offset(0),
    expected_return(""),
    variables(std::unordered_map<string, string>())
  {};
  State(const State &other):
    offset(other.offset),
    expected_return(other.expected_return),
    variables(other.variables)
  {};
};

#endif
