cmake_minimum_required(VERSION 3.25)

project(dnelibs C)

set(CMAKE_C_STANDARD 11)

add_subdirectory(lib/framebuffer/sys)
add_subdirectory(clib/stdlib)