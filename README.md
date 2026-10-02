<img width="240px" src="./assets/banner.png" alt="vanda" />

---

vanda is a tiny ARMv7-A Linux kernel bootloader written in Rust. It implements the bare minimum requirements for the [ARM Linux boot protocol](https://docs.kernel.org/arch/arm/booting.html).

vanda is somewhat unique, because it runs entirely as position-independent shellcode, meaning it can execute from any RWX base address. It also doesn't write to any memory outside of its binary, meaning the space it takes up is the space it uses.

> [!INFO]
> If you are booting hardware that requires initialization of DRAM (or any other board-specific hardware needed *before* the kernel boots), you will need to handle that before jumping to vanda. vanda is intentionally minimal and doesn't have drivers. vanda also will not load kernel images from any storage medium for you, that should be done beforehand as well (see the [Payloads](#payloads) section).

<!-- ```
cargo build -p vanda --release && llvm-objcopy -O binary target/armv7a-none-eabihf/release/vanda packages/vanda-trampoline/src/vanda.bin &&
cargo v5 run -p vanda-trampoline --release
``` -->

## Building

```sh
cargo build --release && llvm-objcopy -O binary target/armv7a-none-eabi/release/vanda vanda.bin
```

Try it in QEMU on a cortex-a15:

```sh
qemu-system-arm -M virt -cpu cortex-a15 -m 128M -nographic -device loader,file=vanda.bin,addr=0x0,force-raw=on
```

## Payloads

Payloads (kernel images and devicetree blobs) are added to vanda through simple concatenation.

**Supported kernel images:**

- `zImage`, `bzImage`, `Image`

ATAGs are unsupported. vanda is intended for devicetree targets only.