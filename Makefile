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

# rustc command
RUSTC=rustc

.PHONY: all build run clean

# Rule to build the project
build: $(BINDIR)
	$(RUSTC) $(RUSTC_FLAGS) -o $(BINARY) $(SRC)

# Rule to create the bin directory if it doesn't exist
$(BINDIR):
	mkdir -p $(BINDIR)

# Rule to run the binary
run: build
	./$(BINARY)

# Clean the generated binary
clean:
	rm -rf $(BINARY)

# Build and run the project
all: build run
