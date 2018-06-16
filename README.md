# Description

This is a bitboard-based chess engine with two major goals:

1. Raw performance through focusing on parallel processing and SIMD from the start.
2. Implementation of a Monte Carlo Tree Search (MCTS) using shallow alpha-beta search rather than random playouts.

## Version History
#### v0.1

* Verified 100% accurate legal move generation
* Basic alpha beta search of constant depth 7
* Simple evaluation based on material and one set of piece-square tables

PERFT MNodes/sec:
* starting position: 22.13
* kiwipete: 18.43

FICS Blitz Rating: 1554 (16 games played)
