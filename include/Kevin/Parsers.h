#ifndef Parsers_h
#define Parsers_h

#include "./State.h"
#include "./Error.h"

#define Filter(x) if(!(x)) return false
#define Resolve(x) state = new_state; result = x; return true

bool Eat(string, string, State&, string&);
bool Eat(string, string, State&);
bool EatWhite(string, State&);

bool EatFunction(string, State&, string&);
bool EatFunctionName(string, State&, string&);

bool EatScope(string, State&, string&);
bool EatBracketScope(string, State&, string&);
bool EatExpressionScope(string, State&, string&);

bool EatStatementAssignment(string, State&, string&);

#endif
