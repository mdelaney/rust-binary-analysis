# Rust Binary Analysis
## There are plenty of great binary analysis tools out there, so why create this?
Quite simply, I want to learn binary analysis and Rust. That and I was inspired by a PoC||GTFO article 
"Build Your Own Fucking Birdfeeder" by Manul Laphroaig. This is my birdfeeder (that will hopefully
become something others will find useful in time)
> What I can't create, I do not understand. (Richard Feynman)

This readme will be a kind of living document that I'll update as pieces get implemented or improved.

## What's been done so far
Initially ELF files are only supported. I've done a fair bit of work implementing a parser from the
ground up. I've gotten it to the point that I can work with different sections but there is still
quite a bit to do.

After implementing some basic ELF support, I moved on to writing a wrapper for Capstone. Why not
use an existing wrapper? To build my own birdfeeder and learn a bit about wrapping C libraries :-P.

I've been working through the book "Practical Binary Analysis" by Dennis Andriesse and wanted to
implement the Capstone exercises in Rust (which has led me down a number of rabbit holes that
have taught me quite a bit so far). I've gotten this much working.

## What will I work on next?
There are a few things I'd like to spend time on.
* Better support for ELF files (there is quite a bit I could do here, lots to learn)
* A more fleshed out Capstone wrapper. Support for `cs_disasm_iter` is high on my list.
* Implement support for PE files.
* Look at an abstraction layer on top of ELF and PE files to provide a common interface to work
with binaries.
* Write tests for the above.
