# rabbit-task-dispatcher

## Project Overview
This training project consists of [Rust CLI Client](https://github.com/wezik/rabbit-task-dispatcher/edit/main/README.md#rust-cli-client), RabbitMQ, and [Golang worker](https://github.com/wezik/rabbit-task-dispatcher/edit/main/README.md#golang-worker).  
The primary objective of this one is to challenge myself with message brokers. Additionally, I've opted to user other programming languages, namely Rust and Go.

Simple representative graph:
![graph2](https://github.com/wezik/rabbit-task-dispatcher/assets/68642257/4e10ccee-37fb-4a2c-b317-b78c5af1dc61)
*(created using https://excalidraw.com lovely tool)*

## Continuous Integration and Continuous Deployment (CI/CD)
The project is seamlessly integrated with GitHub Actions, automatically triggering builds and running test suites upon pushes to their corresponding directories. You can review each workflow below:
- [Rust CLI workflow](https://github.com/wezik/rabbit-task-dispatcher/actions/workflows/rust.yml)
- [Go workers workflow](https://github.com/wezik/rabbit-task-dispatcher/actions/workflows/go.yml)

## Rust CLI Client
Requirements:
- Cargo installed

It is under cli directory & it's a standard rust project so  
- head to `cli` directory with `cd cli`

### Build
```
cargo build
```
### Run
```
cargo run
```

## Golang worker
Requirements:
- Go installed
  
It is under Go-worker directory & it's a standard go project so
- head to `go-worker` directory with `cd go-worker`
### Build
```
go mod build ./
```
### Run
```
go mod run ./
```
  
