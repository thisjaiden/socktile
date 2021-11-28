`socktile`
===
An unfinished and in-development game.

Running
---
Dev builds are avalable by going to [Github Actions](https://github.com/thisjaiden/socktile/actions) and downloading build artifcats for your platform.

Building
---
Make sure you have [Rust](https://rust-lang.org) installed.  
Clone the project, and `cargo build --release` in the root directory. (Builds without the `--release` flag are extremely slow.)  
Make sure to run the final executable from `target/release/socktile.exe` with `/assets` alongside it.

Weird Network Issues?
---
Networking is encoded through a system that uses unique build hashes. Unfortunately, this system will silently fail for similar but different systems. Make sure to use the **EXACT SAME** executable if running a local GGS and/or testing.
