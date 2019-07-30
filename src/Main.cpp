#include <iostream>
#include <fstream>
#include "Kevin/Parsers.h"

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
  std::ifstream ifs("../test.kev");
  string source(
    (std::istreambuf_iterator<char>(ifs)),
    (std::istreambuf_iterator<char>())
  );
  std::cout << source << std::endl;
  State state;
  string out;
  Eat(source, "[ \n]*", state);
  EatFunction(source, state, out);
  std::cout << out << std::endl;
  return 0;
}
