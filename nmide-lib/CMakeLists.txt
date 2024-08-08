cmake_minimum_required(VERSION 3.25)

project(
  LibNmide
  LANGUAGES
    C
  VERSION
    0.1.0
)

set(CMAKE_C_STANDARD 11)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

# Testing add_executables
if(CMAKE_BUILD_TYPE STREQUAL Debug)
  include_directories(
  html
  model
  model/test
  map
  map/test
  map/test/test_carr
  map/test/test_cfree
  map/test/test_ckey
  map/test/test_ckeypair
  map/test/test_cmap
  map/test/test_cval
  map/test/test_maybe
  msg/test
  )

  add_executable(
    test_cmap
    map/test/cmap_test_suite.c
    map/test/test_carr/test_carr.c
    map/test/test_cfree/test_cfree.c
    map/test/test_ckey/test_ckey.c
    map/test/test_ckeypair/test_ckeypair.c
    map/test/test_cmap/test_cmap.c
    map/test/test_cval/test_cval.c
    map/test/test_maybe/test_maybe.c
    munit/munit.c
    model/cmodel.c
    map/cmap.c
  )

  add_executable(
    test_cmodel
    model/test/test_cmodel.c
    model/cmodel.c
    munit/munit.c
    map/cmap.c
  )

  add_executable(
    test_cmsg
    msg/test/test_cmsg.c
    msg/cmsg.c
    munit/munit.c
    map/cmap.c
  )
endif()

# Create the static library
if(CMAKE_BUILD_TYPE STREQUAL Release)
  include_directories(
  html
  model
  map
  msg
  interface
  )

  add_library(nmide STATIC
  nmide.c
  html/html.c
  model/cmodel.c
  map/cmap.c
  )

  install(TARGETS nmide DESTINATION .)
endif()

