# SigmaOS-mkinitcpio-archiso — Documentation

## Purpose

Initramfs hooks for live ISO builds

This component mirrors the responsibility of `archlinux/mkinitcpio-archiso` inside the
SigmaOS ecosystem, implemented in memory-safe Rust.

## Architecture

- `src/lib.rs` — public API and core logic
- `tests/` — integration tests
- Follows separation of mechanism (this crate) and policy (configuration).

## Engineering principles applied

Abstraction, component isolation, loose coupling, high cohesion, KISS, DRY,
secure defaults, fail-fast, defensive programming, deterministic testing,
stable APIs, open standards.
