# Cellular Automata

## Synopsis

The purpose of this project is:
- to provide an easily reusable interface for creating videos from a series of 2D-vectors describing each frame
- to adapt such an interface with the goal of simulating discrete bidimentional cellular automata on rectangular or torus grids
- to implement specific such automata

## Preview

![Brian's brain](img/brain_capture.gif)

## How to build and run

- make sure that `ffmpeg` is installed
- edit main to choose automata and setup (it will eventually be possible to load a configuration file to avoid this step)
- compile with cargo (release mode recommended) or use `rsmake` to compile and copy the executable to the root directory (make sure that `cargo` is in your `$PATH`)
- execute
- open the resulting `.avi` video


## Work in progress and future improvements

Work is currently in progress on:
- Abelian sandpiles (https://en.wikipedia.org/wiki/Abelian_sandpile_model)
- Conway's game of life and any other life-like automata (https://en.wikipedia.org/wiki/Life-like_cellular_automaton)
- Brian's brain (https://en.wikipedia.org/wiki/Brian%27s_Brain)
- Langton's ant (https://en.wikipedia.org/wiki/Langton%27s_ant)


TODO:
- enable reading from text files to initialize grid (beta available for game of life)
- create scanner to load a screenshot of a game of life state into an initializer


It should be noted that this project relies heavily on `ffmpeg`. Luckily `ffmpeg` is available for all OS, but the commands may need tweaks to execute properly on non-Linux distributions.
