//! Pellis runtime-trunk core.
//!
//! `no_std` and alloc-free by design (see the runtime-trunk design spec). This crate
//! is currently just the buildable shell — the four seams (Skill / Model / Device /
//! Trace) and the Governor land in subsequent PRs, each behind its own interface.
#![no_std]
