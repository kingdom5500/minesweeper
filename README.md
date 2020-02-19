# minesweeper
a text-based implementation of minesweeper written in rust.

## usage

### difficulty

to play a **preset difficulty** (beginner, intermediate, expert) run
```sh
./minesweeper [beginner|intermediate|expert]
```

to play a **custom game**, provide dimensions/mines in the form WxH_M.
for example, to play a game of width 15, height 10, with 30 mines,
run the following command:
```sh
./minesweeper custom 15x10_30
```

### controls
- arrows - move cursor
- space - dig a tile
- f - place a flag
- d - perform a chord
- p - pause/unpause
- q - quit game

## preview

![example play](/images/preview.gif)
