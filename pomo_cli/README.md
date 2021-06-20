# Pomo CLI

Pomodoro CLI.


## How to use

```
RUST_LOG=INFO pomo_cli pomo
RUST_LOG=INFO pomo_cli break
```

At this moment you need to put `RUST_LOG=INFO`. It is now under the consideration.

## Development and build

Dev container is prepared. So, what you need to prepare for start is (almost) only Docker and VS Code.

But that container is not prepared for cross-platform build. In other words, when you need to make a binary for Windows or Mac, you need to prepare Rust environment and build in that environment.

