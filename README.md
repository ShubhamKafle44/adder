# Overview

This project implements a compiler for Adder, a minimal language with 32-bit integers and three unary operations: add1, sub1, and negate. The compiler parses input, generates x86-64 assembly, and produces an executable that prints the result.

# My Implementation

I fully implemented the compiler with these components:

Parser: Converts S-expressions into an AST. Supports Num, add1, sub1, and negate. Added error handling for invalid input and integer bounds.

Code Generator: Recursively traverses the AST and generates x86-64 assembly. Implements all operations with results stored in rax.

Runtime: Rust code calls the compiled assembly (our_code_starts_here) and prints the output.

Build Flow: Reads .snek input → parses → compiles → writes .s file. Wrapped assembly with boilerplate for execution. Command-line argument checks ensure safe usage.

Testing: Created multiple test files covering numbers and all operations. Verified outputs and captured results in transcript.txt.

