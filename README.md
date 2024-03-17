# rabbit-task-dispatcher

## Project Overview
This training project consists of [Rust Client](https://github.com/wezik/rabbit-task-dispatcher/tree/main?tab=readme-ov-file#rust-client), RabbitMQ, and [Golang worker](https://github.com/wezik/rabbit-task-dispatcher/tree/main?tab=readme-ov-file#golang-worker).  
The primary objective of this one is to challenge myself with message brokers. Additionally, I've opted to user other programming languages, namely Rust and Go.

Simple representative graph:
![graph2](https://github.com/wezik/rabbit-task-dispatcher/assets/68642257/eea17df5-74e3-4a33-93c7-a5c64a3d37cf)
*(created using https://excalidraw.com lovely tool)*

## Continuous Integration CI
The project is seamlessly integrated with GitHub Actions, automatically triggering builds and running test suites upon pushes to their corresponding directories. You can review each workflow below:
- [Rust client workflow](https://github.com/wezik/rabbit-task-dispatcher/actions/workflows/rust.yml)
- [Go workers workflow](https://github.com/wezik/rabbit-task-dispatcher/actions/workflows/go.yml)

## Running project
Requirements:
- Cargo installed
- Go installed
- RabbitMQ installed and running

1. Configure RabbitMQ in [.env](https://github.com/wezik/rabbit-task-dispatcher/blob/main/client/.env) file or use default one
2. Run [Rust Client](https://github.com/wezik/rabbit-task-dispatcher/tree/main?tab=readme-ov-file#rust-client) and any amount of [Golang worker](https://github.com/wezik/rabbit-task-dispatcher/tree/main?tab=readme-ov-file#golang-worker)'s
3. Use said client to interact with everything
  
### Rust Client
It is under client directory & it's a standard rust project so  
- head to `client` directory with `cd client`

#### Build
```
cargo build
```
#### Run
```
cargo run
```

### Golang worker
It is under Go-worker directory & it's a standard go project so
- head to `go-worker` directory with `cd go-worker`
#### Build
```
go build ./
```
#### Run
```
go run ./
```
  
