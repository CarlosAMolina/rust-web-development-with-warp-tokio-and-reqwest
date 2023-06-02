## Introduction

This is the project realized in the book `Rust Web Development: With warp, tokio, and reqwest`.

## Theory

### Runtime

The runtime is at the center of the asynchronous web service.

The runtime is responsible for:

- Handling multiple computations at the same time: creating threads, polling our futures, and driving them to completion.
- Provide an abstraction over the async kernel API: passing on work to the kernel and making sure to use the asynchronous kernel API to not have any bottlenecks there as well.

Rust doesn't have a runtime or abstraction over asynchronous kernel APIs (for example, Node.js and Go come with a native runtime and abstraction over the kernel API). Rust gives us the syntax and a type and understands asynchronous concepts, we have to build or use a runtime.

Example: Tokio. Tokio uses a crate called Mio (https://github.com/tokio-rs/mio) for the asynchronous communication with the operating system kernel.

### Web framework

The web framework abstracts over the HTTP implementation, server, and runtime so we can focus on writing business logic for the application.

What it does:

- Route handler.
- Session handling.
- Parsing URL parameters.
- Etc.

Example: Actix Web, Rocket, Warp, Axum.

## Configuration

Configure the Bad Words API: <https://github.com/CarlosAMolina/bad-words>.

## Run

```bash
cargo run
make get-questions
make add-question
```

## Resources

Book code:

<https://github.com/Rust-Web-Development/code>

Theory

<https://livebook.manning.com/book/rust-web-development/chapter-2>
