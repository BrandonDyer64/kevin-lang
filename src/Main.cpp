#include <iostream>
#include "./KSL.h"
#include "./Eat.h"
#include "./Function.h"

string source = R"u8(

fn i32 Main() =>

)u8";

i32 main() {
  u32 offset = 0;
  string out;
  Eat(source, "[ \n\r]*", offset);
  Function(source, offset, out);
  std::cout << out << std::endl;
  std::cout << u8"Hello, World!" << std::endl;
  return 0;
}
