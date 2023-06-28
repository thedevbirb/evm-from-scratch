# EVM From Scratch Challenge

## What is the EVM?
The EVM is the core of the Ethereum protocol. It is a stack-based virtual machine that executes 
bytecode and updates the global state according to the rules described in the Ethereum Yellow Paper. 
The EVM is responsible for executing smart contracts and is what makes Ethereum a "World Computer".

## What is this challenge about?
[w1nt3r-eth](https://github.com/w1nt3r-eth/evm-from-scratch) is the creator of the [EVM From Scratch challenge](https://github.com/w1nt3r-eth/evm-from-scratch), which consists in a series of 152 tests 
that need to be passed sequentially in order to have a simpler (yet almost complete) working EVM implementation.

## About my implementation 
I tried to remain as close as possible to the [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf), which 
accurately describes how the EVM should be implemented. Starting from types to the execution model, having an almost one-to-one
correspondence to the paper helped me a lot to follow a right path.  

This is not my first attempt, but the second. If you want to see some crappy Rust code, take a look at [evm-from-scratch-failed](https://github.com/lorenzofero/evm-from-scratch-failed).
During my first attempt I tried to gain a grasp of what I had to do, without focusing too much on details. Also, I had very little knowledge of the Rust language, 
and the Yellow Paper is very tough on a first read.  

However, this is my first real Rust project and I'm very happy with the result because I learned a lot while doing it.

## Credits
- [w1nt3r-eth](https://github.com/w1nt3r-eth/evm-from-scratch) for creating the [EVM From Scratch challenge](https://github.com/w1nt3r-eth/evm-from-scratch), 
- [merklefruit](https://github.com/merklefruit) because I copied just a bit his readme file of his [challenge](https://github.com/merklefruit/evm-from-scratch) 
and for some moral support :)
