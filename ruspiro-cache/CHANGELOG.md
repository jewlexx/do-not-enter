# Changelog

## :peach: v0.4.1

This is mainly a maintenance version migrating the build pipeline to GitHub Actions.

## :peach: v0.4.0

This version removes the aarch32 support from the crate and introduces the use of a better pipeline config to build and publish this crate.

- ### :wrench: Maintenance

  - refactor the crate and remove aarch32 support
  - remove the assembly code and use rust code to do the cache maintenance operations with a bit of inline assembly
  - introduce the new pipeline configuration

## :banana: v0.3.1
  
- ### :wrench: Maintenance

  - use `cargo make` to stabilize build
  - remove `asm` feature as the crate does not use inline assembly