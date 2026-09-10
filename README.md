# stardew-emulator

Rust terminal emulation engine for Stardew, powered by alacritty_terminal.

## Overview

This crate handles ANSI escape sequence parsing, terminal grid state, PTY management, and produces a plain-data `TerminalSnapshot` that the renderer consumes.

The shell is Stardew's own bash, not Android's toybox shell. It loads from the Stardew prefix and refuses to start if the bootstrap has not been installed.

## Dependencies

- alacritty_terminal for VT100/xterm parsing and grid management
- portable-pty for pseudo-terminal handling
- jni for Android bindings

## Building

```toml
cargo build --features "jni"
```

## Testing

```toml
cargo test --features "jni"
```

## Architecture

- `src/engine.rs` - terminal state and escape sequence processing
- `src/pty.rs` - PTY creation, shell spawning, I/O
- `src/snapshot.rs` - plain-data types for the renderer
- `src/config.rs` - paths and environment for the Stardew shell
- `src/jni.rs` - JNI bridge for Android
- `src/input.rs` - key encoding for terminal sequences

## Shell Requirement

The emulator spawns the shell at:

```bash
/data/data/io.stardew/files/usr/bin/bash
```

This file is created by the Stardew bootstrap. Until it exists, `StardewEmulator::new` returns an error.

## License

Apache-2.0
