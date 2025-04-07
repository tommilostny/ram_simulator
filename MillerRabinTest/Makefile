# Compiler and flags
NVCC = nvcc
NVCC_FLAGS = -O3

# Project files
TARGET = isprime
SRC = MillerRabinTest/kernel.cu
OBJ = $(SRC:.cu=.o)

# Default target
all: $(TARGET)

# Compile CUDA source file
%.o: %.cu
	$(NVCC) $(NVCC_FLAGS) -c $< -o $@

# Link the executable
$(TARGET): $(OBJ)
	$(NVCC) $(NVCC_FLAGS) $^ -o $@

# Run the executable
run: $(TARGET)
	./$(TARGET)

# Clean up
clean:
	rm -f $(OBJ) $(TARGET)

.PHONY: all run clean