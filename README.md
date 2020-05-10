# Cellular Automata

## Synopsis

The purpose of this project is:
- to provide an easily reusable interface for creating videos from a series of 2D-vectors describing each frame
- to adapt such an interface with the goal of simulating discrete bidimentional cellular automata on rectangular or torus grids
- to implement specific such automata

## Preview

![Brian's brain](img/brain_capture.gif)

Brian's brain

## How to build and run

- make sure that `ffmpeg` is installed
- edit main to choose automata and setup (it will eventually be possible to load a configuration file to avoid this step)
- compile and run with cargo: `cargo run --release` (`--release` mode is recommended since the time lost optimizing is easily compensated during the execution, see NOTE #1)
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


## NOTE #1: About runtime vs compile-time

We first compare build times:
```
$ cargo build
   Compiling cellular_automata v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.27s
$ cargo build --release
   Compiling cellular_automata v0.1.0
    Finished release [optimized] target(s) in 1.94s
```

for both of these, the dependencies were compiled first, then all files of the project were forcefully recompiled. The difference between `dev` and `release` modes is noticeable, even more so when only main is recompiled: 0.79s vs 1.98s.
More tests yielded comparable results: between 0.58s and 0.81s for `dev`, against

Thus compiling in `dev` rather that `release` can spare us a bit over a second.

But what of runtime ?

Both builds were run with the following configuration:
- Lifelike(23/3) rules
- a 200x300 field
- a single bricklayer loaded at the top right corner
- 5000 generations

The results speak for themselves:

`dev`: 12m28s (incl. 45s running `ffmpeg`)

`release`: 1m29s (incl. 45s running `ffmpeg`)
