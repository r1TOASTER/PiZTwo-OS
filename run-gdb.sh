# Make sure aarch64-none-elf-gdb requires exposed, per session
export PYTHONHOME=/usr/lib/python3.8
export PYTHONPATH=/usr/lib/python3.8

# Run architecure's gdb with custom script
aarch64-none-elf-gdb -x qemu_connect.gdb