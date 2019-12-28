# texel_types

Texel Types base crate containing only types and their basic functions.

This crate is meant to provide serializable data types for use with programs wishing to use scenes created with [Texel ASCII art editor](https://github.com/almindor/texel)

## [Documentation](https://docs.rs/texel_types/1.0.0/)

## Features

* `serde_support` - adds [serde](https://crates.io/crates/serde) and [serde_derive](https://crates.io/crates/serde_derive) as dependencies for serialization support
* `ecs_spec` - adds [specs](https://crates.io/crates/specs) `Component` support for all top level types for use with ECS (since both the types and traits would be foreign if used).

## Forward Compatibility

The types in this crate are meant to be forward compatible serialize-safe.

In practice it means that future versions of the `Scene` wrapper enum will
never lose data when converting previous version of themselves*¹*.

### Notes
*¹*: This promise is currently broken in V1, see [documentation](https://docs.rs/texel_types/1.1.0/texel_types/struct.SceneV1.html) for an explanation.