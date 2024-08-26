cmake_minimum_required(VERSION 3.25)

project(
  LibNmideCPlugin
  LANGUAGES
    C
  VERSION
    0.1.0
)

set(CMAKE_C_STANDARD 11)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

# Create the static library
if(CMAKE_BUILD_TYPE STREQUAL Release)
  add_library(nmide-plugin-c SHARED
    main.c
  )

  find_library(NMIDE_PATH
    NAMES nmide
    PATHS ${CMAKE_CURRENT_SOURCE_DIR}/../../nmide-lib/release
    NO_DEFAULT_PATH
  )
  if (NMIDE_PATH)
    message(STATUS "Library found at ${MYLIB_PATH}")
  else()
    message(FATAL_ERROR "Library not found")
  endif()

  target_link_libraries(nmide-plugin-c ${NMIDE_PATH})

  install(TARGETS nmide-plugin-c DESTINATION .)
endif()

