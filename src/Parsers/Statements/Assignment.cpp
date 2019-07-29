#include "Kevin/Parsers.h"

bool EatStatementAssignment(string source, State &state, string &result) {
  State new_state(state);
  string letvar;
  string type;
  string name;
  Filter(Eat(source, "(let|var)", new_state, letvar));
  Filter(Eat(source, " +", new_state));
  if (!Eat(source, "[a-z][a-z0-9_]*", new_state, name)) {
    if (Eat(source, "[0-9][a-z0-9_]+", new_state, name)) {
      Error(source, new_state, name.length(), "Variables don't start with numbers.");
      return false;
    }
    if (Eat(source, "[A-Za-z0-9_]+", new_state, name)) {
      Error(source, new_state, name.length(), "Variables don't have capital letters.");
      return false;
    }
    Filter(Eat(source, "[a-zA-Z0-9_]+", new_state, name));
    Error(source, new_state, name.length(), "Invalid Variable Name");
    return false;
  };
  Filter(Eat(source, " *", new_state));
  if (!Eat(source, ";", new_state)) {
    Error(source, new_state, 1, "This isn't a semicolon");
    return false;
  }
  Resolve("%" + name + " = x");
}
