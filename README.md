# Fancy title
Thanks for having a look at this side-project of mine. It's the first thing I've made that I'm proud, and excited, to share. 

Youtube playlist of demos [here](https://www.youtube.com/playlist?list=PLrriSbGy4itgS2tNGbgveFjb-8Tf1CnDN&fbclid=IwAR1ycQVb1wZEr-Wvt88IrNlIXRAsMfT7I61Vg9etkCPbW8HDAkdq9Y8p2lI)

If you're interested, I'm currently looking for mentors in Rust, game-dev, and writing and using shaders (or any combination of those). I can't guarantee anything in return beyond gratitude, but I love to teach and mentor, so maybe we can find something.

To me, this is a significant accomplishment. At the start, I knew:
 * Relatively little Rust
 * Nothing about shaders - what they are, how they are used, how they are coded, etc.
 * Very little about running something outside the terminal
 * How to use a game engine

I have made significant progress on all of these, and I don't plan to stop.

# Installation

## Build tools
This builds with rust. If you don't have rust you can find instructions [here](https://www.rust-lang.org/tools/install). If you don't have them already, you'll need `curl` and `git` most likely

Also need the dev-tools for the game engine. [See reference if needed](https://github.com/ggez/ggez/blob/master/docs/BuildingForEveryPlatform.md)

#### Linux dev-tools
substitute `apt` for your platforms package manage

`apt install libasound2-dev libudev-dev pkg-config`


#### Mac and Windows dev tools
Should work out of the box.

#### S H A D E R    P O W E R 

Current shader assumes a reletively recent card/on-board graphics. [See issue](https://github.com/Ben-PH/shader_brot/issues/13)

# Running
Assuming no curve-balls in the process, all you need to do to run this is to clone it, then `cargo run` in the project root directory. Any technical, TODO-list, or documentation suggestions to make this process easier for you is highly-welcomed.

#### For added effect

Have [this](https://www.youtube.com/watch?v=zHU2RlSCdxU) audio playing in the background. I do not take any responsibility for resulting desires to join the hypno-cult.

## Controls
- E to zoom in
- D to zoom out
- arrow-keys to move around
- q to quit, with the bonus of `MainState` struct details printed to stdout
- TAB to switch between Julia set, MandelBrot set, and hybrid.
- S to decrease detail (Increases performance)
- W to increase detail (decreases performance)

# Overview

## Why doesn't it do `thing`? That would be awesome!
Probably! feel free to add an issue, but I have another side-project that I'm working on. I'll always put aside some time to look at a pull-request, though, and help get it in if it fits the project :D

## What's going on?
At time of writing, it's a real-time, shader-powered fractal renderer with exploration. It currently supports the MandelBrot set, and some related interactive fractals.

## Why can I not zoom infinately? Shouldn't fractals do that?
The system currently calculates to 64-bit precision, which is not infinately precise. When things get zoomed in deep enough, the lack of infinate precision means that we eventually cannot have a value for a pixels _actual_ location, and so we must round it out. There are ways around this, but implementing it is beyond the scope of my _current_ interests.
