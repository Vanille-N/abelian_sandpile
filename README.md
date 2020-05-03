# Cellular Automata

The purpose of this project is:
- to provide an easily reusable interface for creating videos from a series of 2D-vectors describing each frame
- to adapt such an interface with the goal of simulating discrete bidimentional cellular automata on rectangular or torus grids
- to implement specific such automata

Work is currently in progress on:
- Abelian sandpiles (https://en.wikipedia.org/wiki/Abelian_sandpile_model)
- Conway's game of life and any other life-like automata (https://en.wikipedia.org/wiki/Life-like_cellular_automaton)
- Brian's brain (https://en.wikipedia.org/wiki/Brian%27s_Brain)
- Langton's ant (https://en.wikipedia.org/wiki/Langton%27s_ant)


Planned modifications:
- enable reading from text files to initialize grid


It should be noted that this project relies heavily on `ffmpeg`: unless an easily usable alternative is found, portability to non-Linux systems is not an objective.
