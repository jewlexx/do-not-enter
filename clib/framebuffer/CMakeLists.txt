cmake_minimum_required(VERSION 3.25)
project(framebuffer C)

set(CMAKE_C_STANDARD 11)

add_library(framebuffer src/framebuffer.c include/framebuffer.h)

target_link_libraries(framebuffer PiStdlib)