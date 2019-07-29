#include <iostream>
#include "Kevin/Parsers.h"

string source = R"u8(

fn i32 Main() {
  let a;
}

)u8";

i32 main()
{
  // float progress = 0.0;
  // while (progress <= 1.0)
  // {
  //   int barWidth = 70;
  //
  //   printf("% *d", 4, int(progress * 100.0));
  //   std::cout << " %\033[1;94m  ━ ";
  //   int pos = barWidth * progress;
  //   for (int i = 0; i < barWidth; ++i) {
  //     if (i < pos) std::cout << "█";
  //     //else if (i == pos) std::cout << "▒";
  //     else std::cout << "░";
  //   }
  //   std::cout << "\r\033[0m";
  //   std::cout.flush();
  //
  //   progress += 0.00001; // for demonstration only
  // }
  // std::cout << std::endl;
  State state;
  string out;
  Eat(source, "[ \n\r]*", state);
  EatFunction(source, state, out);
  std::cout << out << std::endl;
  return 0;
}
