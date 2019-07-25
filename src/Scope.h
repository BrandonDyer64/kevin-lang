#ifndef Scope_h
#define Scope_h

#include "./Eat.h"
#include "./KSL.h"

bool BracketScope(string source, u32 &offset, string &result) {
  u32 newoffset = offset;
  Filter(Eat(source, "\\{[ \n\r]*", offset));
  Filter(Eat(source, "\\}[ \n\r]*", offset));
  Resolve("BRACKET");
}

bool ExpressionScope(string source, u32 &offset, string &result) {
  u32 newoffset = offset;
  Resolve("EXPRESSION");
}

bool Scope(string source, u32 &offset, string &result) {
  u32 newoffset = offset;
  string content;
  Filter(
    BracketScope(source, offset, content) ||
    ExpressionScope(source, offset, content)
  );
  Resolve("\n{\n" + content + "\n}\n");
}

#endif
