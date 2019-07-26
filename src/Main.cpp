#include <iostream>
#include "Kevin/Parsers.h"

string source = R"u8(

fn i32 Main() {
  let Invalid = 7;
}

)u8";

i32 main()
{
  State state;
  string out;
  Eat(source, "[ \n\r]*", state);
  EatFunction(source, state, out);
  std::cout << out << std::endl;
  std::cout << u8"Hello, World!" << std::endl;
  return 0;
}
