# Define variables
CC = clang
CFLAGS = -c
AR = ar
ARFLAGS = rcs
BINDGEN = bindgen
RUSTC = rustc

C_LIB_DIR = mybind
SRC_DIR = src
BINDINGS_FILE = $(SRC_DIR)/bindings.rs
LIBRARY = $(C_LIB_DIR)/libtest.a

# C source files
SRC = $(C_LIB_DIR)/test.c $(C_LIB_DIR)/include/test2.c $(C_LIB_DIR)/include2/test3.c $(C_LIB_DIR)/library/biblioteca.c 
OBJ = $(SRC:.c=.o)

# Rust source files
RUST_SOURCES = $(SRC_DIR)/main.rs

# Targets
all: $(LIBRARY) $(BINDINGS_FILE) $(RUST_SOURCES)
	$(RUSTC) $(RUST_SOURCES) -l static=test -L $(C_LIB_DIR)

$(BINDINGS_FILE): $(C_LIB_DIR)/test.h
	$(BINDGEN) $(C_LIB_DIR)/test.h -o $(BINDINGS_FILE)

# $(C_OBJECTS): $(C_SOURCES)
# 	$(CC) $(CFLAGS) $(C_SOURCES) -o $(C_OBJECTS)

# $(LIBRARY): $(C_OBJECTS)
# 	$(AR) $(ARFLAGS) $@ $(C_OBJECTS)

# Compile object files
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Create static library
$(LIBRARY): $(OBJ)
	$(AR) $(ARFLAGS) $@ $^

clean:
	rm -f $(C_OBJECTS) $(LIBRARY) $(BINDINGS_FILE) main $(C_LIB_DIR)/*/*.o 
