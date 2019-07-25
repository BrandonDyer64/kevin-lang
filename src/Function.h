#ifndef Function_h
#define Function_h

#include "./KSL.h"
#include "./Eat.h"
#include "./Scope.h"

bool FunctionName(string source, u32 &offset, string &result) {
  u32 newoffset = offset;
  string head;
  string tail;
  Filter(Eat(source, "[A-Z]", newoffset, head));
  Filter(Eat(source, "[a-zA-Z0-9]+", newoffset, tail));
  Resolve(head + tail);
}

bool Function(string source, u32 &offset, string &result) {
  u32 newoffset = offset;
  string type;
  string name;
  string scope;
  Filter(Eat(source, "fn", newoffset));
  Filter(Eat(source, " +", newoffset));
  Filter(Eat(source, "[a-zA-Z0-9]+", newoffset, type));
  Filter(Eat(source, " +", newoffset));
  Filter(FunctionName(source, newoffset, name));
  Filter(Eat(source, "\\(\\) ", newoffset));
  Filter(Scope(source, newoffset, scope));
  Resolve(type + " " + name + "() " + scope);
}

#endif
