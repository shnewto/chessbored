# chessbored

a chess board with movable pieces that behaves like a chessboard on the table. useful for solo tinkering, setting up puzzles, and analyzing positions. FEN notation is generated for positions on the board and can be copied with a mouse click and pasted into engines for further analysis.

![grey and white chess board, grey and white pieces each in their starting positions and a selection menu of each piece along the right edge of the board. below the board are white letters on a black background describing the positions on the board in FEN notation along with a note that you can click to copy the FEN description.](/img/board.png)

## controls

- left mouse click: pickup / place a piece
- x: remove a selected piece
- c: clear the board
- i: all pieces in "initial" / starting positions

## try it out

<https://chessbored.shnewto.space>

## running locally

```rust
cargo run
```
