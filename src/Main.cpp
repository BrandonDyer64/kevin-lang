#include <iostream>
#include "./KSL.h"
#include "./Eat.h"
#include "./Function.h"

string source = R"u8(

fn i32 Main() {

}

)u8";

i32 main() {
  int offset = 0;
  Eat(source, "[ \n\r]*", offset);
  auto out = Function(source, offset).value();
  std::cout << out << std::endl;
  std::cout << u8"Hello, World!" << std::endl;
  return 0;
}
