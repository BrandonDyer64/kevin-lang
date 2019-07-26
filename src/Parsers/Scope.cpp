#include "Kevin/Parsers.h"

bool EatBracketScope(string source, State &state, string &result)
{
  State new_state(state);
  string out;

  Filter(Eat(source, "\\{[ \n\r]*", new_state));
  Filter(EatStatementAssignment(source, new_state, out));
  Filter(Eat(source, "[ \n\r]*", new_state));
  Filter(Eat(source, "\\}", new_state));
  Resolve(out);
}

bool EatExpressionScope(string source, State &state, string &result)
{
  State new_state(state);
  Filter(Eat(source, "=>", new_state));
  Resolve("return x;");
}

bool EatScope(string source, State &state, string &result)
{
  State new_state(state);
  string content;
  Filter(
    EatBracketScope(source, new_state, content) ||
    EatExpressionScope(source, new_state, content)
  );
  Resolve("\n{\n" + content + "\n}\n");
}
