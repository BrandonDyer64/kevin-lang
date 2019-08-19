cmake_minimum_required(VERSION 3.0)
set(CMAKE_BUILD_TYPE Release)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17 -Wfatal-errors")

project(kevin DESCRIPTION "Kevin Language")

find_package(LLVM REQUIRED CONFIG)

message(STATUS "Found LLVM ${LLVM_PACKAGE_VERSION}")
message(STATUS "Using LLVMConfig.cmake in: ${LLVM_DIR}")

file(GLOB_RECURSE SOURCES src/*.cpp src/*.c)

add_executable(kevin ${SOURCES})

target_include_directories(
  kevin PUBLIC include
)

target_include_directories(kevin PUBLIC ${LLVM_INCLUDE_DIRS})
add_definitions(${LLVM_DEFINITIONS})

llvm_map_components_to_libnames(llvm_libs support core irreader)

target_link_libraries(kevin ${llvm_libs})

# Use "sudo make install" to apply
install(TARGETS kevin
  DESTINATION /usr/bin
)
