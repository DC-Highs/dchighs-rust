# DC Highs for Rust

Rust ports of the [DC Highs](https://github.com/DC-Highs) Dragon City libraries, built for high performance.
Fan-made, not affiliated with SocialPoint.

| crate | port of | purpose |
| --- | --- | --- |
| [`dc-core`](crates/dc-core) | `@dchighs/dc-core` | shared enums, game settings, validation tools and calculators |
| [`dc-localization`](crates/dc-localization) | `@dchighs/dc-localization` | translations fetching, key lookup, compare and object translation |
| [`dc-statistics`](crates/dc-statistics) | `@dchighs/dc-statistics` | generate item, dragon, island, and global game statistics |
| [`dc-assets`](crates/dc-assets) | `@dchighs/dc-assets` | download sprites, thumbnails, animations, music |
| [`dc-config`](crates/dc-config) | `@dchighs/dc-config` | fetch/load the game configuration |
| [`dc-config-mapper`](crates/dc-config-mapper) | `@dchighs/dc-config-mapper` | restructure, translate and populate the configuration |
| [`dc-client-state`](crates/dc-client-state) | `@dchighs/dc-client-state` | Windows client state manager (UserDefault.xml preferences and cached assets) |

## Using it before publishing

```toml
[dependencies]
dc-core = { path = "D:/v2Projects/@dchighs/rust-packages/crates/dc-core" }
dc-localization = { path = "D:/v2Projects/@dchighs/rust-packages/crates/dc-localization" }
dc-statistics = { path = "D:/v2Projects/@dchighs/rust-packages/crates/dc-statistics" }
dc-assets = { path = "D:/v2Projects/@dchighs/rust-packages/crates/dc-assets" }
dc-config = { path = "D:/v2Projects/@dchighs/rust-packages/crates/dc-config" }
dc-config-mapper = { path = "D:/v2Projects/@dchighs/rust-packages/crates/dc-config-mapper" }
dc-client-state = { path = "D:/v2Projects/@dchighs/rust-packages/crates/dc-client-state" }
```

Or from a git repository: `cargo add --git https://github.com/<you>/<repo> dc-core dc-localization dc-statistics`.

## Publishing to crates.io

Dependencies must exist on the registry first, so publish in this order (`cargo login` once):

```sh
cargo publish -p dc-core
cargo publish -p dc-localization
cargo publish -p dc-statistics
cargo publish -p dc-assets
cargo publish -p dc-config
cargo publish -p dc-config-mapper
cargo publish -p dc-client-state
```

After that: `cargo add dc-core dc-localization dc-statistics dc-assets dc-config dc-config-mapper dc-client-state`.

Before publishing, check the `repository` URL in the root `Cargo.toml` (it currently points to a placeholder
`DC-Highs/dchighs-rs`) and bump `version` there (all crates share it).

## Development

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
DC_CONFIG_FIXTURE=/path/to/config.json cargo test -p dc-config-mapper --test real_config -- --nocapture
```
