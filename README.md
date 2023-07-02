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

### Concurrently vs parallel

- Concurrency: makes progress on more than one task at the same time, can have the effect of starting and pausing tasks while finishing them. With `tokio::join!`, the futures (HTTP calls) are executed concurrently on the same thread, both of them make progress at the same time (for example by context switching).
- Parallelism: more resources are being created or used to work simultaneously on the given tasks. Example, `tokio::spawn` creates another task on the same thread or creates a new thread, allowing parallel execution.

You have to evaluate what option gives a better performance.

### Authentication and authorization

#### Registration endpoint

It allows to send an email and password combination to the server.

#### Login endpoint

The login endpoint is the entrance to the application, sends back a token that is the key to authorizing HTTP requests.

#### Middleware

A middleware is placed before the HTTP request is passed to the route handler.

It's job is to add information to a request, so the route handlers can do their job. For example, it adds the parameter account ID.

#### Rout handlers

They extract information from the request and check for a valid token to know if a client is allowed to modify the resource.

A token is a stateless way of authentication. You can have a database table with active tokens to invalidate tokens in the future.

### File build.rs

Cargo will run the code in the build.rs file before it compiles the application code.

Usage examples:

- Set environment variables to use in the CI/CD pipeline. For example, if we include in this environment variable the hash of the current Git commit and log it each time the application is started, this will help to identify the commit when a bug appears.

Resources:

- <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-env>

## Configuration

Configure the Bad Words API: <https://github.com/CarlosAMolina/bad-words>.

## Run

Initialize the database, read the `db/README.md` file.

Initialize the server:

```bash
cd server
cargo run
cd ..
```

Create an account:

```bash
make add-account
```

Login to get a token:

```bash
make add-account
```

Copy the retrieved token and place it in the `Authorization` headers of the makefile file.

Now we can create a question:

```bash
make add-question
```

To update the question, maybe you have to update the question id in the makefile, after that, run:

```bash
make update-question
```

The makefile contains other requests to use. For example:

```bash
make get-questions
```

## Resources

Book code:

<https://github.com/Rust-Web-Development/code>

Theory

<https://livebook.manning.com/book/rust-web-development/chapter-2>
