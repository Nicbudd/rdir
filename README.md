# rdir - 🔥Blazingly Fast🚀 📁mkdir📁 ✨Replacement✨ Written in 🦀Rust🦀

rdir is a memory safe 🔥Blazingly Fast🚀 replacement for mkdir written entirely in Rust.

## Features:

- Memory safe directory writing
- 🔥Blazingly Fast🚀 execution speed (under 1 ms!)
- mkdir is old and stale and unmaintained. It has been around for 40 years, and the project needs some fresh eyes.
- Written in the most loved programming language according to the 2022 Stack Overflow Developer Survey
- Alternative for mkdir incase it is broken or has a backdoor.

## Todo
- Optimize code to make it even more faster
- ✨**AI**✨ integration
- Figure out funding/monetization
- Replace backdoor with a slightly less obvious backdoor.

## Benchmarks

`mkdir`
```
real    12m5.352s
user    2m53.947s
sys     9m11.603s
```

✨`rdir`✨
```
real    0m0.002s
user    0m0.001s
sys     0m0.001s
```
## Installing

Installation is simple:
```
mkdir ~/untrusted_github_repos
cd ~/untrusted_github_repos
git clone https://github.com/Nicbudd/rdir
cd rdir
cargo build --release
```
Running is as simple as:

```
~/untrusted_github_repos/rdir/target/release/rdir <directory>
```

or 

```
~/untrusted_github_repos/rdir/target/release/rdir -p <directory>
```