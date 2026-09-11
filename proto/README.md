# Vendored steam-vent protobuf crates

These crates are vendored from https://codeberg.org/steam-vent/proto and
regenerated for `protobuf =3.7.2` (the published 0.5.x crates pin
`protobuf =3.5.1`, which is affected by RUSTSEC-2024-0437).

| Crate                     | Version | Upstream commit                            |
|---------------------------|---------|--------------------------------------------|
| `steam-vent-proto`        | 0.5.2   | `953b1b24bfbd41930951de6ed77f9998a580ee8c` |
| `steam-vent-proto-common` | 0.5.1   | `890d27a421e8d671c7ef52ef17192ad46bc2ae66` |
| `steam-vent-proto-steam`  | 0.5.2   | `8634a446730e3c24cce72f3086b89ef1570aaba2` |

The commits are the ones recorded in the published crates'
`.cargo_vcs_info.json`. Changes relative to the published crates:

- `common/Cargo.toml`: `protobuf` bumped from `=3.5.1` to `=3.7.2`.
- `steam/src/generated/`: regenerated from the unchanged `steam/protos/` with
  upstream's `build/` generator at `8634a44`, with its `protobuf`,
  `protobuf-codegen` and `protobuf-parse` pins bumped to `=3.7.2`. Only the
  `rust-protobuf` version header and `VERSION_3_7_2` check differ.
- `Cargo.toml` / `src/lib.rs` of `steam-vent-proto`: the optional
  `tf2`/`csgo`/`dota2` game crates and features were removed, because the
  published game crates depend on `protobuf =3.5.1` and cannot be resolved
  alongside 3.7.2. The upstream `[workspace]` section (which only excluded
  the non-vendored `build/` directory) was dropped as well.

Upstream ships no license file; the crates are declared `license = "MIT"`
(author: Robin Appelman). The `.proto` files originate from
https://github.com/SteamDatabase/Protobufs.

To regenerate, check out upstream at `8634a44`, bump the `protobuf*` pins in
`build/Cargo.toml` to `=3.7.2`, and run
`cargo run -- <this repo>/proto/steam/protos <this repo>/proto/steam/src/generated`
from `build/`.
