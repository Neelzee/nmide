CC = gcc
CFLAGS = -Wall -Wextra -O2 -fPIC
LDFLAGS = -shared
AR = ar
ARFLAGS = rcs
TARGET_LIB = nmidelib.so
STATIC_LIB = nmidelib.a

# Source files
SRCS = nmidelib.c html/css_lib.c
HDRS = nmidelib.h html/css_lib.h html/html_lib.h

OBJS = $(SRCS:.c=.o)

# Default target
all: $(TARGET_LIB) $(STATIC_LIB)

# Build the shared library
$(TARGET_LIB): $(OBJS)
	$(CC) $(LDFLAGS) -o $@ $^

# Build the static library
$(STATIC_LIB): $(OBJS)
	$(AR) $(ARFLAGS) $@ $^

# Compile source files into object files
%.o: %.c $(HDRS)
	$(CC) $(CFLAGS) -c $< -o $@

# Clean up build artifacts
clean:
	rm -f $(OBJS) $(TARGET_LIB) $(STATIC_LIB)

.PHONY: all clean
