cmake_minimum_required(VERSION 3.15)

project(LibNmide C)


# Include directories
include_directories(html munit test)

# Create the static library
add_library(nmide STATIC
  nmide.c
  html/html.c
  html/css.c
  munit
)

# Define the test executable and link it with the library
add_executable(nmide_test  test/test_nmide.c)

# Install the library
install(TARGETS nmide DESTINATION .)

