# OS
OS made in Rust following https://os.phil-opp.com/

Runs on QEMU x86-64.

## Setup emulation environment

Cargo automatically runs everything on QEMU, standard run and tests.

- Download [QEMU](https://www.qemu.org/)
- Install Rust with Rustup
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Install Nightly toolchain

## Execution

In order to execute that, make sure you have done `rustup update` and `cargo update`, to be sure that everything is updated.

You can run kernel with the command `cargo run`, tests can be done with `cargo test` or `cargo test --test test_name` so to execute 
a specific test target.

