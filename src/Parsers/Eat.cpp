#include <algorithm>
#include <regex>
#include "Kevin/Parsers.h"

bool Eat(string source, string regex, State &state, string &result)
{
  std::regex base_regex(regex);
  std::smatch base_match;
  string newsource = source.substr(state.offset);
  if (!regex_search(newsource, base_match, base_regex)) return false;
  if (base_match.size() == 0) return false;
  if (base_match.position(0) != 0) return false;
  state.offset += base_match[0].length();
  result = base_match[0];
  return true;
}

bool Eat(string source, string regex, State &state)
{
  string a;
  return Eat(source, regex, state, a);
}

bool EatWhite(string source, State &state)
{
  string content;
  if (Eat(source, "[ \n]+", state, content)) {
    state.line_num += std::count(content.begin(), content.end(), '\n');
    return true;
  }
  return false;
}
