# Chesscalc

A program to calculate the highest possible score by moving across a weighted chessboard according to legal chess piece moves.

## Logic

Consider the example board:

```
00 01 02 03 04
05 06 07 08 09
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24
```

Assume a piece is starting at index 2.

The moveset rules are as follows:

- A `Pawn` can move only to cell 7.
- A `Bishop` can move to cells 6 and 8.
- A `Knight` can move to cells 5 and 9.
- A `King` can move to cells 6, 7, and 8.

## Example

```
    Board:
13 22 13 13
18 20 04 10
12 22 01 24
20 08 05 01


COL #0 SUMS:
    Pawn   63
    Bishop 53
    Knight 34
    King   75

COL #1 SUMS:
    Pawn   72
    Bishop 82
    Knight 55
    King   84

COL #2 SUMS:
    Pawn   23
    Bishop 53
    Knight 52
    King   75

COL #3 SUMS:
    Pawn   48
    Bishop 59
    Knight 65
    King   59
```
