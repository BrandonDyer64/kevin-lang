cmake_minimum_required(VERSION 3.0)
set(CMAKE_BUILD_TYPE Release)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17 -Wfatal-errors")

project(kevin DESCRIPTION "Kevin Language")

file(GLOB_RECURSE SOURCES src/*.cpp src/*.c)

add_executable(kevin ${SOURCES})

target_include_directories(
  kevin PUBLIC include
)

# Use "sudo make install" to apply
install(TARGETS kevin
  DESTINATION /usr/bin
)
