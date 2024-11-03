//! Comprehensive example of the generated builder and its typestate API.
//!
//! The preliminary reading of [Typestate API](https://bon-rs.com/guide/typestate-api)
//! guide is recommended to understand how the pieces in this example fit together.
//!
//! This module contains a struct [`Example`] that was annotated with [`#[derive(Builder)]`](crate::Builder).
//! The config [`#[builder(state_mod(vis = "pub"))]`](https://bon-rs.com/reference/builder/top-level/state_mod)
//! was applied to make the generated builder's typestate API public and visible here in the docs.
//!
//! The following was generated by the macro:
//! - [`ExampleBuilder`] - the builder struct itself
//! - [`example_builder`] - the builder's typestate API module

/// Example struct with the `#[derive(Builder)]` annotation.
#[derive(crate::Builder)]
#[builder(crate = crate, state_mod(vis = "pub"))]
pub struct Example {
    /// Example required member
    x1: u32,

    /// Example optional member
    x2: Option<u32>,

    /// Example member with a default value.
    #[builder(default = 2 + 2)]
    x3: u32,
}