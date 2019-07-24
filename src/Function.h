#ifndef Function_h
#define Function_h

#include "./KSL.h"
#include "./Eat.h"

#define Filter(x) if(!x) return false

bool Function(string source, int &offset, string &result) {
  int newoffset = offset;
  string type;
  string name;
  Filter(Eat(source, "fn", newoffset));
  Filter(Eat(source, " +", newoffset));
  Filter(Eat(source, "[a-zA-Z0-9]+", newoffset, type));
  Filter(Eat(source, " +", newoffset));
  Filter(Eat(source, "[a-zA-Z0-9]+", newoffset));
  offset = newoffset;
  result = type + " " + name + "()\n{}";
  return true;
}

#endif
