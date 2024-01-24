# sl-up
An interactive TUI smartlog for [Sapling SCM](https://github.com/facebook/sapling), like git-sl-up but written in Rust.

This is my own personal implementation for a tool available internally at Meta. FWIW, I do not have access to that tool's code so if they choose to open source the internal tool at any time, you should prefer that one over mine. I would ;) 

I've ported this from my [Python version of the tool](https://github.com/vuduchild/sl-up-py) as an exercise to learn Rust. It's the first I've ever written in this exciting language, sorry if I made some unexperienced choices. Happy for any feedback!

# Installation
The easieset way would be using cargo, which would install the latest release from [crates.io](https://crates.io/crates/sl-up):
```
$ cargo add sl-up
```

Otherwise, you can of course clone this repo and build it yourself with:
```
$ git clone https://github.com/vuduchild/sl-up && cd sl-up && cargo build --release
```
