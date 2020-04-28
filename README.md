# Cellular Automata

The purpose of this project is:
- to provide an easily reusable interface for creating videos from a series of 2D-vectors describing each frame
- to adapt such an interface with the goal of simulating discrete bidimentional cellular automata on rectangular or torus grids
- to implement specific such automata

Work is currently in progress on:
- Abelian sandpiles
- Conway's game of life
- Brian's brain

Next versions may include:
- Langton's ant


Planned modifications:
- generalize to any Life-like automata
- enable reading from text files to initialize grid


It should be noted that this project relies heavily on `ffmpeg`: unless an easily usable alternative is found, portability to non-Linux systems is not an objective.
