# Makefile for NASM assembly program

# Name of your asm file without extension
TARGET ?= idk

# Tools
ASM = nasm
LD = ld

# Flags
ASMFLAGS = -f elf32
LDFLAGS = -m elf_i386

# Default rule
all: $(TARGET)

# Build executable from asm
$(TARGET): $(TARGET).o
	$(LD) $(LDFLAGS) -o $@ $^
	chmod +x $@

# Assemble .asm into .o
$(TARGET).o: $(TARGET).asm
	$(ASM) $(ASMFLAGS) -o $@ $<

# Clean up
clean:
	rm -f $(TARGET) $(TARGET).o
