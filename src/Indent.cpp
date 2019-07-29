#include "Kevin/Parsers.h"

string &Indent(string &source)
{
  string search = "\n";
  string replace = "  ";
  auto pos = source.find(search);

  while (pos != string::npos)
  {
    source.replace(pos, search.size(), replace);
    pos = source.find(search, pos + replace.size());
  }
  source = replace + source;
  return source;
}
