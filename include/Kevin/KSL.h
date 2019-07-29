#ifndef Kevin_Standard_Library_h
#define Kevin_Standard_Library_h

/*

  Kevin Standard Library
  ======================

  By: Brandon Dyer

*/

// ----- Primitives -----

#include <cstdint>
#include <cstddef>

typedef std::uint8_t u8;
typedef std::uint16_t u16;
typedef std::uint32_t u32;
typedef std::uint64_t u64;

typedef std::int8_t i8;
typedef std::int16_t i16;
typedef std::int32_t i32;
typedef std::int64_t i64;

typedef std::size_t i0;

typedef std::int32_t symbol;

// ----- String -----

#include <string>

using std::string;

// ----- Console -----

namespace Console
{
string Print(string x);
}

#endif