# Fancy title
Thanks for having a look at this side-project of mine.

If you're interested, I'm currently looking for mentors in Rust, game-dev, and writing and using shaders (or a combination of those). I can't guarantee anything in return beyond gratitude, but I love to teach and mentor, so maybe we can find something. 

### Installation and running
make sure you have the latest rust installed.

Current shader assumes a reletively recent card/on-board graphics. [See issue](https://github.com/Ben-PH/shader_brot/issues/13)

Assuming no curve-balls in the process, all you need to do to run this is to clone it, then `cargo run` in the project root directory. Any technical, TODO-list, or documentation suggestions to make this process easier for you is highly-welcomed.


## Controls
- E to zoom in
- D to zoom out
- arrow-keys to move around
- q to quit, with the added bonus of `MainState` struct details printed to stdout
- TAB to switch between Julia set and MandelBrot set
- S to decrease detail (Increases performance)
- W to increase detail (decreases performance

### What's going on?
At time of writing, it's a shader-powered mandelbrot-set renderer, with a per-frame refresh. I started this because I wanted to get more into rust, do some stuff that wasn't only inside the terminal, and get a handle on interactive stuff.
