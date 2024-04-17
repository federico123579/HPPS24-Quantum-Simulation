= Introduction

Abstract of the project:
Quantum Computers provide an exponential speedup with respect to classical computation over a large number of use cases. Yet, many of them need an excessive amount of resources to solve problem instances of practical utility. Due to the current limitations of quantum hardware, in terms of number of qubits and effects of noise, simulation is necessary. In this work, we aim to devise a quantum simulation toolchain that enables users to run custom quantum circuits, introduce a noise model, and achieve practical speedup with respect to current platforms, using heterogeneous architectures such as FPGAs.

To simulate quantum circuits on classical computers we use the tensor contraction approach, in which a quantum circuit is contracted up to a single expression of tensor operations (contractions and expansions).

= Notation

- DFS: depth first search
- I: identity matrix
- TC: tensor contraction; it's a matrix multiplication (\*)
- TE: tensor expansion; it's a tensor product (x)

= Example

To further clarify the procedure, let's take a look at a sample circuit.

== Quantum Circuit

In Figure 1 we can observe the representation of the quantum circuit; it has tree lanes (0, 1 and 2) and 4 gates. 

#figure(
  image("images/Example-QC block diagram.svg", format: "svg", width:50%),
  caption: [The quantum circuit in consideration],
)


== Contraction Graph

The contraction graph relative to the above quantum circuit is the one in Figure 2. Note that each node is a gate and we adopt the notation A[n,m], where A is the name of the gate and n and m are the lanes on which the gate acts.
The (directed) arcs are the lanes that directly connect two nodes. Note that the input gates (the ones direcly connected to the input lanes) have no incoming arc and the output gates (the ones direcly connected to the output lanes) have no outgoing arc.

#figure(
  image("images/Example-Contraction Graph 1.svg", format: "svg", width:50%),
  caption: [The initial contraction graph],
)

In Figure 2 we can observe the first step of the contraction, in which gates C and E are contracted in one common gate through 1 TC and 1 TE; in detail first the gate E (which has rank 1) is expanded to a gate with rank 2 by a TE with I; then it's tensor contracted with C. Note that the previous TE was necessary in order to perform the TC by having both C and E in the correct (same) dimension.

#figure(
  image("images/Example-Contraction Graph 2.svg", format: "svg", width:50%),
  caption: [First contraction between nodes C and E],
)

TODO: Note inner join

== Complete Tensor Network Expression

If we continue the procedure listed above, we'll arrive to the following expression, which represents the contraction of the whole initial quantum circuit through TEs and TCs.

```
(A[0,1] ~ (C[1,2] ~ E[2])) ~ D[0,1]
```

== Contraction Tree

We can rewrite the above expression by expanding the operations (adding I when needed) and insering them in a treem, as shown in Figure 4.

#figure(
  image("images/Example-Contraction Tree.svg", format: "svg", width:50%),
  caption: [Contraction Tree],
)

== Instruction List
With a recursive exploration of the tree (which is basically an in-order DFS) we can obtain the list of instructions, in the format:
N: S, I1, I2
where N is the instruction number, S is the operator (\* for TC and x for TE) and Ii are the operands (they can either be other instructions or directly the gates identifiers).

For the circuit under consideration the instruction list is:

TODO: add instruction list

== Dependencies and Dependants

From the instruction list, we can add the list of dependencies and dependants.

The dependency list contains for each instruction, the list of instructions on which it depends.

TODO: add dependency list

The dependecy list contains for each instruction, the list of instructions that depend on it. 

TODO: add dependant list

== ISA and below (hardware)