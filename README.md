`socktile`
===
An unfinished game.

Running
---
Dev builds are avalable by going to [Github Actions](https://github.com/thisjaiden/socktile/actions) and downloading build artifacts for your platform.

Building
---
Make sure you have [Rust](https://rust-lang.org) installed.  
Clone the project, and run `cargo build --release` in the root directory. (Builds without the `--release` flag are extremely slow.)  
You can run a local gameserver by running the output executable with `server` as an argument.

Quick Builds
---
`build_quick_test.bat` / `build_quck_test.sh` are provided to quickly run a local game and server. `build_wasm_test.sh` is avalable to try experimental WASM support, which runs a local game on http://localhost:8080. There is no Windows script for this test. `build_ios_test.sh` is avalable to try experimental iOS support. iOS support is not 100% confirmed and probably poor.

Code Layout & Docs
---
- [JSON Variants](docs/json.md)
- [Asset Layout](docs/asset_layout.md)

License
---
License information can be found in `LICENSE.md` for all original code in this repository.

socktile uses:

| Name | Purpose | License |
| ---- | ------- | ------- |
| [Twemoji](https://twemoji.twitter.com/) | Icons and placeholder textures | [CC-BY 4.0](https://creativecommons.org/licenses/by/4.0/) |
| [Rust](https://rust-lang.org) | Programming language | [Apache 2.0 / MIT](https://www.rust-lang.org/policies/licenses) |
| [Trunk](https://trunkrs.dev/) | WASM building | [MIT](https://github.com/thedodd/trunk/blob/master/LICENSE-MIT) |
| [Bevy](https://bevyengine.org/) | Game engine | [MIT](https://github.com/bevyengine/bevy/blob/main/LICENSE-MIT) |
| [bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader) | Boilerplate extension for `Bevy` | [MIT](https://github.com/NiklasEi/bevy_asset_loader/blob/main/LICENSE-MIT) |
| [bevy_embeded_assets](https://github.com/vleue/bevy_embedded_assets) | Embedding extension for `Bevy` | [MIT]() |
| [bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio) | `Kira` extension for `Bevy` | [MIT](https://github.com/NiklasEi/bevy_kira_audio/blob/main/LICENSE-MIT) |
| [bevy_prototype_debug_lines](https://github.com/Toqozz/bevy_debug_lines) | Debugging extension for `Bevy` | [MIT](https://github.com/Toqozz/bevy_debug_lines/blob/master/LICENSE) |
| [bevy_easings](https://github.com/vleue/bevy_easings) | Animation extension for `Bevy` | [MIT]() |
| [serde](https://serde.rs/) | Data serialization library | [MIT](https://github.com/serde-rs/serde/blob/master/LICENSE-MIT) |
| [serde_json](https://github.com/serde-rs/json) | JSON Extension for `serde` | [MIT](https://github.com/serde-rs/json/blob/master/LICENSE-MIT)
| [bincode](https://github.com/bincode-org/bincode) | Data serialization library | [MIT](https://github.com/bincode-org/bincode/blob/trunk/LICENSE.md) |
| [anyhow](https://github.com/dtolnay/anyhow) | Error propagation | [MIT](https://github.com/dtolnay/anyhow/blob/master/LICENSE-MIT) |
| [ldtk_rust](https://github.com/estivate/ldtk_rust) | [LDtk](https://ldtk.io/) file support | [MIT](https://github.com/estivate/ldtk_rust/blob/master/license.md) |
| [tracing](https://github.com/tokio-rs/tracing) | Backtrace handling | [MIT](https://github.com/tokio-rs/tracing/blob/master/LICENSE) |
| [tracing-wasm](https://github.com/storyai/tracing-wasm) | WASM extension for `tracing` | [MIT](https://github.com/storyai/tracing-wasm/blob/master/LICENSE-MIT) |
| [tracing-subscriber](https://github.com/tokio-rs/tracing/tree/master/tracing-subscriber) | Utility extension for `tracing` | [MIT](https://github.com/tokio-rs/tracing/blob/master/tracing-subscriber/LICENSE) |
| [tracing-log](https://github.com/tokio-rs/tracing/tree/master/tracing-log) | Terminal extension for `tracing` | [MIT](https://github.com/tokio-rs/tracing/blob/master/tracing-log/LICENSE) |
| [rand](https://github.com/rust-random/rand) | Random number generation | [MIT](https://github.com/rust-random/rand/blob/master/LICENSE-MIT) |
| [num](https://github.com/rust-num/num) | Generic number types | [MIT](https://github.com/rust-num/num/blob/master/LICENSE-MIT) |
| [netty](https://github.com/thisjaiden/netty) | Networking | Owner |

All sublibraries of the above may have their own licenses which should all fall under fair use or be otherwise accounted for.
