# Fancy title
Thanks for having a look at this side-project of mine.

Youtube playlist of demos [here](https://www.youtube.com/playlist?list=PLrriSbGy4itgS2tNGbgveFjb-8Tf1CnDN&fbclid=IwAR1ycQVb1wZEr-Wvt88IrNlIXRAsMfT7I61Vg9etkCPbW8HDAkdq9Y8p2lI)

If you're interested, I'm currently looking for mentors in Rust, game-dev, and writing and using shaders (or a combination of those). I can't guarantee anything in return beyond gratitude, but I love to teach and mentor, so maybe we can find something. 

## Installation and running

### Build tools
This builds with rust. If you don't have rust you can find instructions [here](https://www.rust-lang.org/tools/install). If you don't have them already, you'll need `curl` and `git` most likely

Also need the dev-tools for the game engine. [See reference if needed](https://github.com/ggez/ggez/blob/master/docs/BuildingForEveryPlatform.md)

#### Linux dev-tools
substitute `apt` for your platforms package manage

`apt install libasound2-dev libudev-dev pkg-config`


#### Mac and Windows dev tools
Should work out of the box.

### S H A D E R    P O W E R 

Current shader assumes a reletively recent card/on-board graphics. [See issue](https://github.com/Ben-PH/shader_brot/issues/13)

### Running
Assuming no curve-balls in the process, all you need to do to run this is to clone it, then `cargo run` in the project root directory. Any technical, TODO-list, or documentation suggestions to make this process easier for you is highly-welcomed.

#### For added effect

Have [this](https://www.youtube.com/watch?v=zHU2RlSCdxU) audio playing in the background. I do not take any responsibility for resulting desires to join the hypno-cult.

## Controls
- E to zoom in
- D to zoom out
- arrow-keys to move around
- q to quit, with the ahttps://www.rust-lang.org/tools/installdded bonus of `MainState` struct details printed to stdout
- TAB to switch between Julia set, MandelBrot set, and hybrid.
- S to decrease detail (Increases performance)
- W to increase detail (decreases performance)

### What's going on?
At time of writing, it's a shader-powered mandelbrot-set renderer, with a per-frame refresh. I started this because I wanted to get more into rust, do some stuff that wasn't only inside the terminal, and get a handle on interactive stuff.
