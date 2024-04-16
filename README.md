# Tasks


- [X] Find meaningful benchmarks on which we may conduct comparisong
  - [ ] better if state of the art
  - [X] benchmarks can be computed in finding the optimal contraction tree
- [ ] evaluate whether considering an optimization approach on the tree or not
  - [ ] if yes, which optimization approach? Genetic algorithm? Simulated annealing? others...
- [X] Add OpenQASM support (v2.0 and v3.0?) and others used in benchmarks
  - [X] in order to make comparison easier
- [X] Refactor code to implement multiple distinct lane span for gates (CNOT not adjacent)
- [X] Write clear documentation with code examples and motivations
- [ ] Design a ISA for interacting with the FPGA
  - [ ] in order to activate the kernels from the host
- [ ] Make statistics out of tree contractions of multiple benchmarks
  - [ ] to gather information on the most common patterns

# Changelog

- **14 Apr 2024**
  - Refactor of `Span` class to allow for multiple distinct lane spans for gates (even if not adjacent)
  - Refactor of `Gate` to allow dynamic block sizing whenever the lanes are not adjacent
  - Add new interface fro `Bra` and `Ket` notation
  - Refactor of textual syntax to support multiple distinct lane spans
  - Add automated tests for correctness of implementation (unittesting and integration testing)
    - testing also if contraction path computation is correct
  - Fixed many issues
  - Add complete documentation to the code (run `cargo doc` to compile it in a browsable webpage)
- **15 Apr 2024**
  - Add a notebook to generate random circuit and save it as `.qasm` file (OpenQASM v3.0)
  - Converted some older circuits (textual) to OpenQASM v3.0
- **16 Apr 2024**
  - Add lots of new gates (all listed in standard OpenQASM v3.0 gates file, too many to list)
  - Add support for OpenQASM v3.0 (excluding flow control and functions, but including gate combinations)
    - All randomly generated circuits are compatible to this new version
  - Many fixes
  - Add many tests with `.qasm` files
  - Add script to generate random circuits and save them as `.qasm` files as cli utility
  - Add complete parallel execution of CpuExecutor
