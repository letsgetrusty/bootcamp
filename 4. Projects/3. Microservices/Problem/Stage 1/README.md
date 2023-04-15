# Stage 1

__Overview__

For this stage, we are going to implement a microservice app consisting of 2 services, an authentication service and a health check service that monitors the auth service. We will also build a client that can communicate with the auth service.

The services will communicate via [gRPC](https://grpc.io/) & [Protocal Buffers (A.K.A Protobufs)](https://protobuf.dev/). gRPC is a Remote Procedure Call (RPC) framework that allows us to call functions implemented on a remote service the same way we would call functions implemented locally. Protobufs are a way to serialize structured data. Similar to XML or JSON except a lot smaller, faster, and more powerful. We will use Protobufs to define our gRPC services. Once our Protobufs are defined we can automatically generate Rust code that will implement the services.

## Third Party Libraries

Rust has a minimal runtime, which means we will need to use several third-party libraries to implement our project.

### tokio

[tokio](https://tokio.rs/) is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing network applications. We discussed Tokio in the advanced section of the Bootcamp. Re-visit that section if you need a refresher.

### tonic, prost, and tonic-build

[tonic](https://crates.io/crates/tonic) is a Rust implementation of gRPC. It is composed of three main components: the generic gRPC implementation, the high performance HTTP/2 implementation and the codegen powered by [prost](https://crates.io/crates/prost).

[tonic-build](https://crates.io/crates/tonic-build) is a development dependency that we use inside our build script (see `build.rs` below) to compile proto files via `prost` and generate service stubs and proto definitions for use with tonic.

### pbkdf2 & rand_core

[pbkdf2](https://crates.io/crates/pbkdf2) and [rand_core](https://crates.io/crates/rand_core) are used to hash passwords.

### uuid

[uuid](https://crates.io/crates/uuid) is used to generate unique identifies for each user. It is also used within tests to generate unique strings.

### clap

[clap](https://crates.io/crates/clap) is a command-line parser. It will be used to create our stand alone client.

## Project Structure

This microservice project will be done inside a [mono-repo](https://www.atlassian.com/git/tutorials/monorepos) which will make development easier.

```text
/[root folder]
    |__ proto
        |__ authentication.proto
    |__ src
        |__ /auth-service
            |__ main.rs
        |__ /client
            |__ main.rs
        |__ /health-check-service
            |__ main.rs
    |__ build.rs
    |__ Cargo.toml
```

**proto/authentication.proto**

This is where we define our authentication service using Protocal Buffers.

**build.rs**

This file is a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) that we use to compile our Protocal Buffers into Rust code. Cargo will run this build script before compiling our source code.

**auth-service/main.rs**

This is the entry point of the authentication service.

**health-check-service/main.rs**

This is the entry point of the health check service.

**client/main.rs**

This is the entry point of the client.

**Cargo.toml**

Thus file defined our 3 binaries crates, `auth`, `client`, and `health-check`. Additionally, all the dependencies required for this project are included as well.

## Steps

### Step 1

__Project setup__

To start this project follow these steps:

1. Create a new Rust project by running `cargo new microservice-project`. This is going to be the mono-repo where the microservices and client are stored.
2. Create a Github repository and add it as the remote repository for the Rust project you created in the previous step.
3. Replace the contents of your Rust project with the files/folders in Step 1.
4. Install protoc: https://grpc.io/docs/protoc-installation/

Your project is now setup. Review all the files and make sure you understand what each one does. Look at the `Third Party Libraries` and `Project Structure` sections above for guidance.

To make sure everything is working, run each binary:
```bash
cargo run --bin auth
```
```bash
cargo run --bin health-check
```
```bash
cargo run --bin client
```

### Step 2

__Implementing the auth service__

Now that the basic structure of our app is laid out, let's implement the authentication service.

The auth service is broken up into the following files
- `auth-service/users.rs` - Contains logic for creating, storing, retrieving, and deleting users.
- `auth-service/sessions.rs` - Contains logic for creating, storing, retrieving, and deleting sessions.
- `auth-service/auth.rs` - This is where we will implement the auth service interface as defined in `authentication.proto`.
- `auth-service/main.rs` - This is where we will create an instance of the auth service and start the gRPC server.

Copy these files to your repository.

Then complete all the TODO items in the following order:
1. TODO items in `auth-service/users.rs`
2. TODO items in `auth-service/sessions.rs`
3. TODO items in `auth-service/auth.rs`
4. TODO items in `auth-service/main.rs`

__NOTE 1:__ Make sure all the tests pass by running `cargo test`.

__NOTE 2:__ Read through all the code to make sure you understand what's going on.

__NOTE 3:__ For simplicity this project stores all data on the heap. If the server is restarted all the user/session info will be wiped.

### Step 3

__Implementing the health check service__

The health-check service will continuously make gPRC requests to the auth service and print out the responses.

Copy over `health-check/main.rs` to your repository.

To implement the health-check service complete all the TODO items in `health-check/main.rs`.

__NOTE 1:__ Make sure all the tests pass by running `cargo test`.

__NOTE 2:__ Read through all the code to make sure you understand what's going on.

__NOTE 3:__ For simplicity the health check service simply logs responses to standard out. Ideally a health check service would implement more robust logging and alerting.

### Step 4

__Implementing the client__

Now that we've implemented our 2 services it's time to create a CLI client that will allow us to send custom requests to the auth service.

To implement the client complete all the TODO items in `client/main.rs`.

__NOTE 1:__ Make sure all the tests pass by running `cargo test`.

__NOTE 2:__ Read through all the code to make sure you understand what's going on.

## Running

After completing steps 1-4 you should have a fully functioning microservice application!

Let's test it out!

__NOTE:__ We are going to use [cargo watch](https://github.com/watchexec/cargo-watch) to automatically restart ours services when source files change, so make sure you have it installed.

First build the project by running `cargo build`.

Then execute the following commands in different terminal windows:

```bash
cargo watch -c -q -w src/auth-service -x "run -q --bin auth"
```
```bash
cargo watch -c -q -w src/health-check-service -x "run -q --bin health-check"
```

__NOTE:__ These commands should be ran in the order above. The auth service needs to be running before launching the health-check service.

You should see both services printing to standard out.

---

Now let's use the client to issue custom requests to the auth service.

To see the commands available run `./target/debug/client help`

Then try signing up a new user, signing in, and signing out.
