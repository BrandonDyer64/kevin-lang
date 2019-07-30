#include "Kevin/Parsers.h"

bool EatFunctionName(string source, State &state, string &result)
{
  State new_state(state);
  string name;
  if (!Eat(source, "[A-Z][a-zA-Z0-9]+", new_state, name)) {
    if (Eat(source, "[a-z][a-zA-Z0-9]+", new_state, name)) {
      Error(source, new_state, name.length(), "Function names start with capital letters.");
      return false;
    }
    return false;
  }
  Resolve(name);
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
  Resolve("define " + type + " @" + name + "() {\n" + scope + "\n}");
}
