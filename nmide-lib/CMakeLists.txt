cmake_minimum_required(VERSION 3.25)

project(
  LibNmide
  LANGUAGES
    C
  VERSION
    0.1.0
)

set(CMAKE_C_STANDARD 23)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

# Include directories
include_directories(
  html
  model
  munit
  test
  map
  map/test
)

# Testing add_executables
if(CMAKE_BUILD_TYPE STREQUAL Debug)
  add_executable(
    nmide_test
    test/test_nmide.c
    test/test_util.c
    html/html.c
    html/css.c
  )

  add_executable(
    test_cmap
    map/test/test_cmap.c
    munit/munit.c
    model/cmodel.c
    map/cmap.c
  )
endif()

# Create the static library
if(CMAKE_BUILD_TYPE STREQUAL Release)
  add_library(nmide STATIC
  nmide.c
  html/html.c
  html/css.c
  munit/munit.c
  model/cmodel.c
  map/cmap.c
)

  install(TARGETS nmide DESTINATION .)
endif()

