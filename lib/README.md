# Build
cmake -DLLVM_VERSION=3.9 -DLLVM_DIR=/usr/lib/llvm-3.9/lib/cmake/llvm/ ..

# Compile Commands
cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=1 ..

# Both
cmake -DLLVM_VERSION=3.9 -DLLVM_DIR=/usr/lib/llvm-3.9/lib/cmake/llvm/ -DCMAKE_EXPORT_COMPILE_COMMANDS=1 ..


