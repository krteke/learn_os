KERNEL_ELF := target/riscv64gc-unknown-none-elf/release/myos
QEMU := qemu-system-riscv64
QEMU_ARGS := -machine virt -nographic -bios default

.PHONY: build run debug objdump duos copy clean

build:
	cargo build --release

run: build
	$(QEMU) $(QEMU_ARGS) -kernel $(KERNEL_ELF)

debug: build
	@echo "Connect GDB: riscv64-linux-gnu-gdb -ex 'target remote :1234' $(KERNEL_ELF)"
	$(QEMU) $(QEMU_ARGS) -kernel $(KERNEL_ELF) -s -S

objdump: build
	llvm-objdump -d $(KERNEL_ELF) | head -80

duos:
	cargo build --release --no-default-features --features=duos

copy: duos
	scp $(KERNEL_ELF) root@10.168.99.1:/boot/

clean:
	cargo clean
