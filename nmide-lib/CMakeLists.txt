cmake_minimum_required(VERSION 3.0)

project(LibNmide VERSION 0.1)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED True)

include_directories(html)

add_library(nmide STATIC
  nmide.c
  html/html.c
  html/css.c
)

install(TARGETS nmide DESTINATION .)

