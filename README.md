# Tasks

- [ ] Find meaningful benchmarks on which we may conduct comparisong
  - [ ] better if state of the art
  - [ ] benchmarks can be computed in finding the optimal contraction tree
- [ ] evaluate whether considering an optimization approach on the tree or not
  - [ ] if yes, which optimization approach? Genetic algorithm? Simulated annealing? others...
- [ ] Add OpenQASM support (v2.0 and v3.0?) and others used in benchmarks
  - [ ] in order to make comparison easier
- [X] Refactor code to implement multiple distinct lane span for gates (CNOT not adjacent)
- [X] Write clear documentation with code examples and motivations
- [ ] Design a ISA for interacting with the FPGA
  - [ ] in order to activate the kernels from the host
- [ ] Make statistics out of tree contractions of multiple benchmarks
  - [ ] to gather information on the most common patterns
