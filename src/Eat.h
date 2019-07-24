#ifndef Eat_h
#define Eat_h

#include <regex>
#include "./KSL.h"
#include <iostream>

bool Eat(string source, string regex, int &offset) {
  string a;
  return Eat(source, regex, offset, a);
}

bool Eat(string source, string regex, int &offset, string &result) {
  std::regex base_regex(regex);
  std::smatch base_match;
  string newsource = source.substr(offset);
  if (!regex_search(newsource, base_match, base_regex)) return false;
  if (base_match.size() == 0) return false;
  if (base_match.position(0) != 0) return false;
  offset += base_match[0].length();
  result = base_match[0];
  return true;
}

#endif
