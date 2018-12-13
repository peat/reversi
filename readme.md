# Reversi Simulator

Reversi game simulator based on the modern rules found here: https://en.wikipedia.org/wiki/Reversi

It does _not_ have an interactive playing mode!

Currently supports:

- Iterating game generators that can create finished games at ~15 microseconds each.
- Left and right depth-first traversal, and non-repeating random generators.
- Importing and exporting game transcripts at any stage of a game.

Take a look at `src/main.rs` for the different examples of how it works.

## Transcripts

Transcripts are in the form of `E6F4E3F6G5D6E7F5C5` -- representing Dark playing `E6` then Light playing `F4` and so on and so forth.

A "pass" move is represented as `PP` to make it easier to parse and understand game flow.

Fun fact: the example transcript above is the shortest possible Reversi game, discovered by Manubu Maruo in 1957.

## Example

```
Replaying Manubu Maruo's 9 move win ...

  a b c d e f g h
1 • • • • • • • •
2 • • • • • • • •
3 • • • • D • • •
4 • • • D D D • •
5 • • D D D D D •
6 • • • D D D • •
7 • • • • D • • •
8 • • • • • • • •

Transcript: E6F4E3F6G5D6E7F5C5
Score: Dark 13, Light 0
Next turn: Complete
```