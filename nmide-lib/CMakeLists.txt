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
  chtml
  cmodel
  cmodel/test
  cmap
  cmap/test
  cmap/test/test_carr
  cmap/test/test_cfree
  cmap/test/test_ckey
  cmap/test/test_ckeypair
  cmap/test/test_cmap
  cmap/test/test_cval
  cmap/test/test_maybe
  cmsg/test
  )

  add_executable(
    test_cmap
    cmap/test/cmap_test_suite.c
    cmap/test/test_carr/test_carr.c
    cmap/test/test_cfree/test_cfree.c
    cmap/test/test_ckey/test_ckey.c
    cmap/test/test_ckeypair/test_ckeypair.c
    cmap/test/test_cmap/test_cmap.c
    cmap/test/test_cval/test_cval.c
    cmap/test/test_maybe/test_maybe.c
    munit/munit.c
    cmodel/cmodel.c
    cmap/cmap.c
  )

  add_executable(
    test_cmodel
    cmodel/test/test_cmodel.c
    cmodel/cmodel.c
    munit/munit.c
    cmap/cmap.c
  )

  add_executable(
    test_cmsg
    cmsg/test/test_cmsg.c
    cmsg/cmsg.c
    munit/munit.c
    cmap/cmap.c
  )
endif()

# Create the static library
if(CMAKE_BUILD_TYPE STREQUAL Release)
  include_directories(
  chtml
  cmodel
  cmap
  cmsg
  cinterface
  )

  add_library(nmide STATIC
  nmide.c
  chtml/chtml.c
  cmodel/cmodel.c
  cmap/cmap.c
  )

  install(TARGETS nmide DESTINATION .)
endif()

