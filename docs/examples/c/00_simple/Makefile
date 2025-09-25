# A simple C application for testing.
# This Makefile finds all .c files in the current directory and all subdirectories,
# compiles them into object files, and then links them into a single library.

# Add the Rust output directory to path. Link statically if static=true is provided
LIB_PATHS ?= ../../../../target/debug
LDFLAGS = -L$(LIB_PATHS)

static ?= false
ifeq ($(static),true)
LDFLAGS += -Wl,-Bstatic
endif
LDFLAGS += -lconfigtpl
ifeq ($(static),true)
LDFLAGS += -Wl,-Bdynamic
endif
LDFLAGS += -lm

# Define the compiler and flags
CXX ?= gcc
CXXFLAGS = -g -O0

# Define the name of the executable to be created
TARGET ?= my_test_app

# Use the 'find' command to locate all .cpp files in the 'src' directory and subdirectories.
# This is a robust way to handle multiple nested folders.
SOURCES = $(shell find . -name "*.c")

# Automatically generate the names of the object files from the source files.
# The `patsubst` function replaces the .cpp suffix with .o
OBJECTS = $(patsubst %.c,%.o,$(SOURCES))


# ----------------------------------------------------------------------------
# Main targets
# ----------------------------------------------------------------------------

# The main target is the executable, which depends on all object files.
# The `-o` flag specifies the output file name.
$(TARGET): $(OBJECTS)
	@echo "Linking library: $(TARGET)..."
	$(CXX) -o $@ $(OBJECTS) $(LDFLAGS)
	@echo "Done."

# A generic rule to compile each .c file into its corresponding .o file.
# The `<` variable refers to the prerequisite (the .c file)
# The `$`@ variable refers to the target (the .o file)
# The `-c` flag tells the compiler to compile and assemble but not to link.
%.o: %.c
	@echo "Compiling $<..."
	$(CXX) $(INC) $(CXXFLAGS) -c $< -o $@

# ----------------------------------------------------------------------------
# Cleanup and utility targets
# ----------------------------------------------------------------------------

# Remove all generated object files and the executable.
.PHONY: clean
clean:
	@echo "Cleaning up build artifacts..."
	$(RM) $(OBJECTS) $(TARGET)
	@echo "Cleanup complete."

# Mark these targets as phony, as they do not correspond to files to be created.
.PHONY: all
all: $(TARGET)
