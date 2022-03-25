# Crust - C-like programming language

Crust is a compiler for a simple general purpose programming language largely inspired by C. The main syntactic difference is the use of a context free grammar to ease parsing.

:warning: This project is in a VERY early stage of development. Features that are talked about in
this README are most likely not yet implemented.

## Syntax
Checkout the `examples` directory for some code snippets.

## Features
Crust's features are pretty minimal as far as high-level programming languages go and should be pretty familiar to people experienced with C. Differences include:
* No included preprocessor
  * Instead of exposing interfaces via header files, Crust code is organized in modules, which can be
  imported by other modules. In that case, it takes heavy inspiration from Rust's module system.
* No typedefs

## Target architecture
The Crust compiler targets the LLVM backend for optimal support of different architectures. Native
backends for x86 and AArch64 are planned for the future.
