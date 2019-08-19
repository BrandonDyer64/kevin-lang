#ifndef State_h
#define State_h

#include <unordered_map>
#include "./KSL.h"

struct State
{
  u32 offset;
  string expected_return;
  u8 nest_level;
  u16 line_num;
  std::unordered_map<string, string> variables;

  State():
    offset(0),
    expected_return(""),
    nest_level(0),
    line_num(1),
    variables(std::unordered_map<string, string>())
  {};
  State(const State &other):
    offset(other.offset),
    expected_return(other.expected_return),
    nest_level(other.nest_level),
    line_num(other.line_num),
    variables(other.variables)
  {};
};

#endif
