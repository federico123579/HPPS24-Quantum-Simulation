# Outline

- **Parsing** (from quantum specification to quantum circuit abstraction)
  - **Lexer**: Tokenize the input quantum specification
  - **Parser**: Syntactic analysis, parse the tokenized input into an abstract syntax tree
  - **Semantic Analysis**: Check the validity of the quantum specification and build a DAG of the quantum circuit
- **Tensor Tree Representation**: make the quantum circuit DAG into a tree structure by partitioning the graph (start by tensor of lowest rank and build up)
  - **Graph Partitioning**: partition the graph starting from contraption of tensor of lowest rank and build up
  - **Tensor Tree**: build a tree structure from the partitioned graph

# Specifications

## Algorithms

### Tensor Tree Representation

- **Input**: Quantum circuit DAG
- **Output**: Tensor tree representation of the quantum circuit

```
tree := empty tensor tree
n := number of inital qubits
k := 2 (tensor rank)
while k <= log2(n) do
  make a partial partition of the graph by selecting contracting tensors of rank k
  if some tensors are left uncontracted then
    k := k + 1
...
```
