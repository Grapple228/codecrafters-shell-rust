# About

This is my solution for Shell challenge

[![progress-banner](https://backend.codecrafters.io/progress/shell/15db20a8-6dc6-4a72-9e51-b0fa156c0ec9)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

In this challenge, you'll build your own POSIX compliant shell that's capable of
interpreting shell commands, running external programs and builtin commands like
cd, pwd, echo and more. Along the way, you'll learn about shell command parsing,
REPLs, builtin commands, and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

## Dev setup

Firstly install `cargo-watch`

```sh
cargo install cargo-watch
```

### For execution app on save, use command

```sh
cargo watch -q -c -w src/ -w .cargo/ -x run
```

### For execution test app on save, use command

```sh
cargo watch -q -c -w examples/ -w .cargo/ -x "run --example from_std"
```

### For execution tests on save, use command

```sh
cargo watch -q -c -x "test -q -- --nocapture"
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
