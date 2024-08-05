cmake_minimum_required(VERSION 3.15)

project(
  LibNmide
  LANGUAGES
    C
  VERSION
    0.1.0
)


# Include directories
include_directories(
  html
  model
  munit
  test
  map
  map/test
)

# Create the static library
add_library(nmide STATIC
  nmide.c
  html/html.c
  html/css.c
  munit/munit.c
  model/cmodel.c
  map/cmap.c
)

# Define the test executable and link it with the library
add_executable(
  nmide_test
  test/test_nmide.c
  test/test_util.c
  html/html.c
  html/css.c
  munit/munit.c
  model/cmodel.c
  map/cmap.c
  map/test/test_cmap.c
)

# Install the library
install(TARGETS nmide DESTINATION .)

