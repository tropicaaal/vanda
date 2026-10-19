<img width="240px" src="./assets/banner.png" alt="vanda" />

---

vanda is a tiny ARMv7-A Linux kernel bootloader[^1] written in Rust. It implements the bare minimum requirements for the [ARM Linux boot protocol](https://docs.kernel.org/arch/arm/booting.html).

vanda is a little unique, because it runs entirely as position-independent shellcode, meaning it can be loaded to and executed at any 32-byte aligned RWX base address. It also doesn't write to any memory outside of its binary, meaning the space it takes up is the space it uses.

[^1]: vanda is more "boot" than "loader", because it doesn't actually load anything from secondary storage for you (see the [Payloads](#payloads) section). It's kind of similar to [m1n1](https://github.com/asahilinux/m1n1) in this regard.

## Why This Exists

This mostly exists for hardware hacking/education purposes and not as a serious attempt at an actual production bootloader, but there are 3 reasons I made this:

1. I used this for my [v5 linux](https://github.com/tropicaaal/buildroot-vex-v5) project to boot the kernel on some reverse-engineered hardware with a lot of unknowns around CPU state and memory layout early into bringup. Having complete control over a small bootloader that can be loaded anywhere gives a lot of flexibility in these scenarios.
2.  There isn't many resources online about bare-metal position-independent Rust code and I thought it would be a fun to try and make it work.
3. Most bootloaders are extremely bloated and do way more than they really should. This makes them both difficult to reason about and increases their attack surface. I wanted to make something that does the *absolute minimum*, mostly as a demonstration of how minimal the boot protocol *can be*. For example, vanda has no heap and does not dynamically allocate.

## Building

```sh
cargo build --release && llvm-objcopy -O binary target/armv7a-none-eabi/release/vanda vanda.bin
```

Try it in QEMU on a cortex-a15:

```sh
cargo build &&
llvm-objcopy -O binary target/armv7a-none-eabi/debug/vanda vanda.bin &&
qemu-system-arm -M virt -cpu cortex-a15 -m 128M -vga none -nographic -kernel vanda.bin -s -S
```

## Payloads

Payloads (kernel images and devicetree blobs) are added to vanda through simple concatenation.

**Supported payload types:**

- Compressed kernel images (`zImage`).
- Compressed initramfs `cpio` archives.
- Flattened device trees (`*.dtb`).
- Uncompressed kernel images (only with the `uncompressed-kernel` feature, must be the last payload in the chain).
<!-- - Flattened image trees (`fitImage`) -->

Uncompressed kernel images and initramfs payloads are unsupported.

Passing device information through ATAGs is unsupported. vanda is intended for devicetree targets only.
