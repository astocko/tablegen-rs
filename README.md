# tablegen-rs

Minimal Rust bindings to LLVM's TableGen via the ctablegen library. Requires llvm-config, LLVM Core and Support libraries along with includes.

# WIP
- Add tests
- Write documentation
- Examples

# Build
If llvm-config is not on the system path or named differently, set the LLVM_CONFIG_PATH environment variable:

LLVM_CONFIG_PATH=/usr/bin/llvm-config-3.9 cargo build

LLVM_CONFIG_PATH=llvm-config-3.8 cargo build
