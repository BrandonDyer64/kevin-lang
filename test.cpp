
#include <cstdint>
#include <iostream>
#include <string>
using std::int32_t;
using std::cout;
using std::endl;
using std::string;
typedef int32_t symbol;

namespace Console {
  using std::cout;
  using std::endl;
  #define Print(x) cout << x << endl
}


int32_t main()
{
  int32_t a = 0x1F3870BE;
  if (a == 0xFE01D67A)
  {
    Console::Print(u8"a is an orange");
  } 
  else
  {
    Console::Print(u8"a is not an orange");
  }
}

