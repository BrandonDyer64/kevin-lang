#include "Kevin/Parsers.h"

bool EatFunctionName(string source, State &state, string &result)
{
  State new_state(state);
  string head;
  string tail;
  Filter(Eat(source, "[A-Z]", new_state, head));
  Filter(Eat(source, "[a-zA-Z0-9]+", new_state, tail));
  Resolve(head + tail);
}

bool EatFunction(string source, State &state, string &result)
{
  State new_state(state);
  string type;
  string name;
  string scope;
  Filter(Eat(source, "fn", new_state));
  Filter(Eat(source, " +", new_state));
  Filter(Eat(source, "[a-zA-Z0-9]+", new_state, type));
  new_state.expected_return = type;
  Filter(Eat(source, " +", new_state));
  Filter(EatFunctionName(source, new_state, name));
  Filter(Eat(source, "\\(\\) ", new_state));
  Filter(EatScope(source, new_state, scope));
  Resolve(type + " " + name + "() " + scope);
}
