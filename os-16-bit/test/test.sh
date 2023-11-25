# ./llvm-objcopy.sh -I elf64-x86-64 -O binary --binary-architecture=i686:x86-64 os-16-bit os-16-bit.bin
# ./llvm-objcopy.sh -I elf64-x86-64 -O binary --binary-architecture=i386:x86-64 os-16-bit os-16-bit.bin

./copy.sh

./llvm-objcopy.sh -I elf64-x86-64 -O binary --binary-architecture=i386:x86-64 os-16-bit os-16-bit.bin
