# Default target
TARGET ?= idk

# Tools
ASM = nasm
LD = ld

# Flags
ASMFLAGS = -f elf32
LDFLAGS = -m elf_i386

# Output binary name (only the filename, no path)
BIN := $(notdir $(TARGET))

# Object file has .o extension and stays next to the .asm file
OBJ := $(TARGET).o
SRC := $(TARGET).asm

all: $(BIN)

# Link object to create final binary
$(BIN): $(OBJ)
	@echo "[LD] Linking $@"
	$(LD) $(LDFLAGS) -o $@ $^
	chmod +x $@

# Assemble .asm to .o
$(OBJ): $(SRC)
	@echo "[ASM] Assembling $< -> $@"
	$(ASM) $(ASMFLAGS) -o $@ $<

clean:
	@echo "[CLEAN] Removing $(BIN) and $(OBJ)"
	rm -f $(BIN) $(OBJ)