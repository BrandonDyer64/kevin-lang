#include "Kevin/Error.h"
#include <iostream>

void Error(string source, State &state, u8 length, string message)
{
  u16 line_num = 1;
  u16 column_num = 0;
  u16 line_offset = 0;
  string line(256, (char) 0);
  bool is_reached_offset = false;
  for (u32 offset = 0; !is_reached_offset || source[offset] != '\n'; offset++) {
    line[offset - line_offset] = source[offset];
    if (offset >= state.offset) {
      is_reached_offset = true;
    } else {
      column_num++;
    }
    if (source[offset] == '\n') {
      column_num = 0;
      line_num++;
      line_offset = offset + 1;
    }
  }
  std::cout << "\033[1;94m";
  for(u8 i = 0; i < 8 + column_num+length + message.length(); i++) {
    std::cout << u8"━";
  }
  std::cout << "\033[0m";
  std::cout << std::endl;
  Console::Print("\033[1;94mError:\033[0m line " + std::to_string(line_num));
  Console::Print("   \033[1;94m┎\033[0m src/example/test.kv");
  Console::Print("   \033[1;94m┃ ");
  printf("% *d", 2, line_num);
  Console::Print(" ┃\033[0m " + line);
  std::cout << "   \033[1;94m┃\033[1;94m " << string(column_num, ' ');
  if (length > 1) {
    std::cout << u8"╹";
    for (u8 i = 0; i < length - 2; i++) {
      std::cout << u8"¯";
    }
    std::cout << u8"╹";
  } else {
    std::cout << u8"╹";
  }
  std::cout << " - " << message << "\033[0m" << std::endl;
}
