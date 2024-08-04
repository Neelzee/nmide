cmake_minimum_required(VERSION 3.15)

project(
  LibNmide
  LANGUAGES
    C
  VERSION
    0.1.0
)


# Include directories
include_directories(html munit test)

# Create the static library
add_library(nmide STATIC
  nmide.c
  html/html.c
  html/css.c
  munit/munit.c
)

# Define the test executable and link it with the library
add_executable(
  nmide_test  test/test_nmide.c  html/html.c  html/css.c  munit/munit.c
)

# Install the library
install(TARGETS nmide DESTINATION .)

