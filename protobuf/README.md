# Protobufs for steam-vent

Sourced from https://github.com/SteamDatabase/Protobufs

## Updating generated code

- cd build
- cargo r -- ../steam/{protos,src/generated}
- cargo r -- ../tf2/{protos,src/generated}
- cargo r -- ../csgo/{protos,src/generated}
- cargo r -- ../dota2/{protos,src/generated}

## Using custom protobufs

If you need to use protobufs that aren't packages by steam-vent, you can create
a new package for the protobufs using the following steps:

1. Create a new crate
2. Add `steam-vent-proto-common` as a dependency
3. Place the protobufs in `src/protos`
4. Create an empty `src/generated` folder
5. Either install `steam-vent-proto-build` trough `cargo-install` or clone this
   repo and build it from the `protobuf/build` directory.
6. Run `steam-vent-proto-build path/to/src/protos path/to/src/generated`
7. Create `src/lib.rs` with the following contents:

   ```rust
   mod generated;

   pub use generated::*;
   ```
