# Source directory
SRC_DIR=./src

# Source file name
SRC=${SRC_DIR}/main.rs

# Directory for the binary
BINDIR=bin

# Directory for the bindings
BINDINGSDIR=bindings

# Name of the binary to be generated
BINARY=$(BINDIR)/my_rust_app

# Compilation flags (optional)
RUSTC_FLAGS=

# bindgen command
BINDGEN = bindgen

# C compile
CC = clang
CFLAGS = -c
AR = ar
ARFLAGS = rcs

# C source files
LIBC_DIR = libc
LIBC_DIR_LIBRARY = libc/library
LIBC_DIR_TEST = libc/test
SRC_LIB = $(LIBC_DIR_LIBRARY)/biblioteca.c \
	$(LIBC_DIR_TEST)/test.c \
	$(LIBC_DIR_TEST)/include/test2.c \
	$(LIBC_DIR_TEST)/include2/test3.c \

OBJ = $(SRC_LIB:.c=.o)

# rustc command
RUSTC=rustc

# library path
LIBRARY_BIN = bin
LIBRARY = $(LIBRARY_BIN)/libtest.a

# bindings path
BINDINGS_FILE = $(BINDINGSDIR)/bindings.rs

.PHONY: all build run clean

# Rule to build the project
build: $(BINDIR) $(LIBRARY) $(BINDINGSDIR) $(BINDINGS_FILE)
	$(RUSTC) $(RUSTC_FLAGS) -o $(BINARY) $(SRC) -l static=test -L $(LIBRARY_BIN)

$(BINDINGS_FILE): $(LIBC_DIR)/wrapper.h
	$(BINDGEN) $(LIBC_DIR)/wrapper.h -o $(BINDINGS_FILE)

# Compile object files
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Create static library
$(LIBRARY): $(OBJ)
	$(AR) $(ARFLAGS) $@ $^

# Rule to create the bin directory if it doesn't exist
$(BINDIR):
	mkdir -p $(BINDIR)

$(BINDINGSDIR):
	mkdir -p $(BINDINGSDIR)

# Rule to run the binary
run: build
	./$(BINARY)

# Clean the generated binary
clean:
	rm -rf $(BINDIR)/* && \
    find . -name "*.o" -type f -delete && \
	rm -rf $(BINDINGSDIR)

# Build and run the project
all: build run
