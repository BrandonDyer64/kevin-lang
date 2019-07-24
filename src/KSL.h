#ifndef Kevin_Standard_Library_h
#define Kevin_Standard_Library_h

/*

  Kevin Standard Library
  ======================

  By: Brandon Dyer

*/

// ----- Primitives -----

#include <cstdint>

typedef std::uint8_t u8;
typedef std::uint16_t u16;
typedef std::uint32_t u32;
typedef std::uint64_t u64;

typedef std::int8_t i8;
typedef std::int16_t i16;
typedef std::int32_t i32;
typedef std::int64_t i64;

// ----- String -----

#include <string>

using std::string;

// ----- Optional -----

#include <optional>

using std::optional;

// ----- Console -----

namespace Console {

string Print(string x) {
  std::cout << x << std::endl;
  return x;
}

}

#endif
