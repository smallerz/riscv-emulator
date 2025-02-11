# Converts a 32-bit hexadecimal integer representing a
# RISC-V instruction into bytes that can be written to
# a file.

# Example:
# ./instr-encoder.sh 0x00558513 >> foo

if [ -z "$1" ]; then
    echo "Usage: $0 <int>"
    exit 1
fi

instr=$1

if [[ "$instr" -lt 0x00 ]] || [[ "$instr" -gt 0xffffffff ]]; then
    echo "Invalid integer - must be between 0x00 and 0xffffffff"
    exit 1
fi

byte0=$((instr & 0xff))
byte1=$(((instr >> 8) & 0xff))
byte2=$(((instr >> 16) & 0xff))
byte3=$(((instr >> 24) & 0xff))

printf "\x$(printf %02x $byte0)\x$(printf %02x $byte1)\x$(printf %02x $byte2)\x$(printf %02x $byte3)"