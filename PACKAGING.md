Packaging
=========

This file contains quick reminders and notes on how to package Bloom.

We consider here the packaging flow of Bloom version `1.0.0` for Linux.

1. **How to bump Bloom version before a release:**
    1. Bump version in `Cargo.toml` to `1.0.0`
    2. Execute `cargo update` to bump `Cargo.lock`
    3. Bump Debian package version in `debian/rules` to `1.0`

2. **How to build Bloom, package it and release it on Crates, GitHub, Docker Hub and Packagecloud (multiple architectures):**
    1. Tag the latest Git commit corresponding to the release `v1.0.0` with tag `1.0` (**this is important!**), and push the tag
    2. Wait for all release jobs to complete on the [actions](https://github.com/valeriansaliou/bloom/actions) page on GitHub
    3. Download all release archives, and sign them locally using: `./scripts/sign_binaries.sh --version=1.0`
    4. Publish a changelog and upload all the built archives, as well as their signatures on the [releases](https://github.com/valeriansaliou/bloom/releases) page on GitHub

Notice: upon packaging `x86_64` becomes `amd64`.
