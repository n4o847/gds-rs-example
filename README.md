# gds-rs-example

This is a reference repository for getting NVIDIA GPUDirect Storage to work in Rust.

## Structure

The folders related in GPUDirect Storage are:

- cufile-sys
  - an FFI wrapper crate for cuFile API
- gds-helloworld
  - a version of the GDS sample program written in Rust

The following folder is a port of Rust-CUDA:

- cpu
  - a host-side crate to perform array addition
- gpu
  - a device-side crate to perform array addition

## Requirements

- MLNX_OFED
- CUDA
- GPUDirect Storage
- Rust
- LLVM 7

## Installation

```bash
git clone https://github.com/n4o847/gds-rs-example.git
```

## Build

```bash
cargo build -p gds-helloworld
```

## Run

```bash
TESTFILE=./foo.txt ./target/debug/gds-helloworld
```

## Resources

I have written an article explaining this repository.

- [GPUDirect Storage を Rust で動かしてみた](https://qiita.com/n4o847/items/07dc24bdb663500970f8)

For others, see also:

- [CUDA Installation Guide for Linux](https://docs.nvidia.com/cuda/cuda-installation-guide-linux/index.html)
- [GPUDirect Storage Installation and Troubleshooting Guide](https://docs.nvidia.com/gpudirect-storage/troubleshooting-guide/index.html)
- [cuFile API Reference Guide](https://docs.nvidia.com/gpudirect-storage/api-reference-guide/index.html)
- [Rust-GPU/Rust-CUDA](https://github.com/Rust-GPU/Rust-CUDA): Ecosystem of libraries and tools for writing and executing fast GPU code fully in Rust.
