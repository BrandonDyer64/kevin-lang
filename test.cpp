
#include <cstdint>
#include <iostream>
#include <string>
using std::int32_t;
using std::cout;
using std::endl;
using std::string;

namespace Console {
  using std::cout;
  using std::endl;
  #define Print(x) cout << x << endl
}


int32_t main() {
  std::function<int32_t(int32_t)> a = [](int32_t x) -> int32_t {
    return 4;
  };
  a(7);
  return 0;
}

