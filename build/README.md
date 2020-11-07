# Requirements

See `.github/workflows/build.yml` for all details. If something is missing from this file, open an issue and/or create a PR to update this doc.

## .NET

Building and testing the C# code requires:

- .NET SDK (latest version): https://dotnet.microsoft.com/download

## Rust

Building and testing the Rust code requires:

- Rust: https://www.rust-lang.org/tools/install
- Extra Rust stuff
	- `rustup update`
	- `rustup component add rustfmt`
	- `rustup component add clippy`
	- `rustup target add wasm32-unknown-unknown`
	- MSRV: `rustup toolchain install 1.20.0`
		- Pass `--no-msrv` to `build-rust.sh` if you don't want to install it
- .NET SDK (latest version): https://dotnet.microsoft.com/download
	- required to generate and test valid/invalid instructions (pass `--no-dotnet` to `build-rust.sh` if you don't want to install .NET)

## JavaScript

Building and testing the JavaScript code requires:

- Rust: https://www.rust-lang.org/tools/install
- Extra Rust stuff
	- `rustup update`
	- `rustup component add rustfmt`
	- `rustup component add clippy`
	- `rustup target add wasm32-unknown-unknown`
- Node.js >= 10.0.0: https://nodejs.org/en/download/
- wasm-pack: `npm install -g wasm-pack`

# Building this repo

Pick an OS, any OS:

## Windows

From the repo root dir:

```cmd
sh build/build.sh
REM It's the same as
sh build/build-rust.sh
sh build/build-js.sh
sh build/build-dotnet.sh
```

`sh` is located in the `git` bin directory.

## Linux / macOS

From the repo root dir:

```sh
./build/build.sh
# It's the same as
./build/build-rust.sh
./build/build-js.sh
./build/build-dotnet.sh
```