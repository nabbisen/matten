//! Dynamic feature tests — split into submodules per the 300-ELOC guideline.
//!
//! All submodules are gated on the `dynamic` feature; each file also gates
//! its internal `mod` blocks individually for finer feature control.

#[cfg(feature = "dynamic")]
mod element;
#[cfg(feature = "dynamic")]
mod guards;
#[cfg(feature = "dynamic")]
mod lifecycle;
#[cfg(feature = "dynamic")]
mod policy;
#[cfg(feature = "dynamic")]
mod tensor;
