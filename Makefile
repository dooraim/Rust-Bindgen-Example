# Source directory
SRC_DIR=./src

# Source file name
SRC=${SRC_DIR}/main.rs

# Directory for the binary
BINDIR=bin

# Name of the binary to be generated
BINARY=$(BINDIR)/my_rust_app

# Compilation flags (optional)
RUSTC_FLAGS=

# C compile
CC = clang
CFLAGS = -c
AR = ar
ARFLAGS = rcs

# C source files
LIBC_DIR_LIBRARY = libc/library
SRC_LIB = $(LIBC_DIR_LIBRARY)/biblioteca.c 
OBJ = $(SRC_LIB:.c=.o)

# rustc command
RUSTC=rustc

# library path
LIBRARY_BIN = bin
LIBRARY = $(LIBRARY_BIN)/libbiblioteca.a

.PHONY: all build run clean

# Rule to build the project
build: $(BINDIR) $(LIBRARY)
	$(RUSTC) $(RUSTC_FLAGS) -o $(BINARY) $(SRC)

# Compile object files
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Create static library
$(LIBRARY): $(OBJ)
	$(AR) $(ARFLAGS) $@ $^

# Rule to create the bin directory if it doesn't exist
$(BINDIR):
	mkdir -p $(BINDIR)

# Rule to run the binary
run: build
	./$(BINARY)

# Clean the generated binary
clean:
	rm -rf $(BINDIR)/*

# Build and run the project
all: build run
