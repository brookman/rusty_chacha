winget install -e --id LLVM.LLVM
$env:RUN_BUILD_RS = '1'; cargo build -r
