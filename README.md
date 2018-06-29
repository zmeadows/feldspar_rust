# Description

This is a bitboard-based chess engine with two major goals:

1. Raw performance through focusing on parallel processing and SIMD from the start.
2. Implementation of a Monte Carlo Tree Search (MCTS) using shallow alpha-beta search rather than random playouts.

# Version History
## v0.1

* Verified 100% accurate legal move generation
* Basic alpha beta search of constant depth 7
* Simple evaluation based on material and one set of piece-square tables

PERFT MNodes/sec:
* starting position: 2.76
* kiwipete: 2.30

ICC Blitz Rating: 1476

#### Line Count
```
-------------------------------------------------------------------------------
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            17            521            337           2934
```

## v0.2

* Added transposition table with Zobrist hashing
* Added Quiescence Search
* Added interpolated mid/end-game Piece-Square-Table evaluation to score
* Switched to iterative deepening search with time management
* Many small improvements to move generation performance

PERFT MNodes/sec:
* starting position: 9.53
* kiwipete: 8.02

ICC Blitz Rating: 1659

#### Line Count
```
-------------------------------------------------------------------------------
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            20            876            430           4418

```
